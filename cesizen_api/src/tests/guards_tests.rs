// ============================================================================
//
// Tests de protection des endpoints CESIZen
// Vérifie que chaque route applique correctement les guards :
//   - RequireAdmin  -> seul un Admin peut accéder
//   - RequireAuth   -> tout utilisateur authentifié peut accéder
//   - Public        -> aucun token nécessaire
//
// Vérifie également :
//   - assert_owns_resource -> ownership / bypass admin
//   - Rejet sans token, avec token invalide, avec mauvais rôle
//
// /!\ Lancer avec --test-threads=1 (set_var n'est pas thread-safe)
// ============================================================================

// ─────────────────────────────────────────────────────────────────────────────
// PARTIE 1 : Tests unitaires purs (guards + claims, sans BDD)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod unit_guards {
    use crate::auth::claims::Claims;
    use crate::auth::guards::assert_owns_resource;
    use crate::auth::{encode_jwt, decode_jwt};
    use crate::modules::user::model::UserRole;

    const TEST_SECRET: &str = "test_secret_suffisamment_long_32chars!";

    /// SAFETY: les tests tournent avec --test-threads=1,
    /// donc pas de data race sur les variables d'environnement.
    fn setup_secret() {
        unsafe { std::env::set_var("JWT_SECRET", TEST_SECRET); }
    }

    fn user_claims(id: i32) -> Claims {
        Claims::new(id, format!("user{}@test.com", id), UserRole::User)
    }

    fn admin_claims() -> Claims {
        Claims::new(1, "admin@cesizen.com".to_string(), UserRole::Admin)
    }

    fn mod_claims() -> Claims {
        Claims::new(2, "mod@cesizen.com".to_string(), UserRole::Mod)
    }

    // ── encode / decode JWT ──────────────────────────────────────────────

    #[test]
    fn jwt_roundtrip_preserves_claims() {
        setup_secret();
        let claims = user_claims(42);
        let token = encode_jwt(&claims).expect("encode devrait réussir");
        let decoded = decode_jwt(&token).expect("decode devrait réussir");

        assert_eq!(decoded.sub, 42);
        assert_eq!(decoded.email, "user42@test.com");
        assert!(matches!(decoded.role, UserRole::User));
    }

    #[test]
    fn jwt_roundtrip_admin_role() {
        setup_secret();
        let claims = admin_claims();
        let token = encode_jwt(&claims).unwrap();
        let decoded = decode_jwt(&token).unwrap();

        assert!(matches!(decoded.role, UserRole::Admin));
    }

    #[test]
    fn jwt_roundtrip_mod_role() {
        setup_secret();
        let claims = mod_claims();
        let token = encode_jwt(&claims).unwrap();
        let decoded = decode_jwt(&token).unwrap();

        assert!(matches!(decoded.role, UserRole::Mod));
    }

    #[test]
    fn decode_rejects_garbage_token() {
        setup_secret();
        assert!(decode_jwt("ceci.nest.pas_un_token").is_err());
    }

    #[test]
    fn decode_rejects_tampered_payload() {
        setup_secret();
        let token = encode_jwt(&user_claims(1)).unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        let tampered = format!("{}.TAMPERED_PAYLOAD.{}", parts[0], parts[2]);
        assert!(decode_jwt(&tampered).is_err());
    }

    #[test]
    fn decode_rejects_wrong_secret() {
        unsafe { std::env::set_var("JWT_SECRET", "secret_A_assez_long_pour_les_jwt!!"); }
        let token = encode_jwt(&user_claims(1)).unwrap();

        unsafe { std::env::set_var("JWT_SECRET", "secret_B_completement_different!!"); }
        assert!(decode_jwt(&token).is_err());

        setup_secret(); // nettoyage
    }

    #[test]
    fn decode_rejects_empty_string() {
        setup_secret();
        assert!(decode_jwt("").is_err());
    }

    // ── assert_owns_resource ─────────────────────────────────────────────

    #[test]
    fn owner_can_access_own_resource() {
        let claims = user_claims(42);
        assert!(assert_owns_resource(&claims, 42).is_ok());
    }

    #[test]
    fn user_cannot_access_other_resource() {
        let claims = user_claims(42);
        assert!(assert_owns_resource(&claims, 99).is_err());
    }

    #[test]
    fn admin_bypasses_ownership_check() {
        let claims = admin_claims();
        assert!(assert_owns_resource(&claims, 99).is_ok());
    }

    #[test]
    fn mod_cannot_bypass_ownership() {
        let claims = mod_claims();
        assert!(assert_owns_resource(&claims, 99).is_err());
    }

    #[test]
    fn mod_can_access_own_resource() {
        let claims = mod_claims();
        assert!(assert_owns_resource(&claims, 2).is_ok());
    }

    // ── Claims::new ──────────────────────────────────────────────────────

    #[test]
    fn claims_new_sets_expiration_in_future() {
        let claims = user_claims(1);
        let now = chrono::Utc::now().timestamp() as usize;
        assert!(claims.exp > now, "exp devrait être dans le futur");
        assert!(claims.exp - claims.iat >= 86000, "durée de validité ~24h");
    }

    #[test]
    fn claims_new_sets_iat_to_now() {
        let before = chrono::Utc::now().timestamp() as usize;
        let claims = user_claims(1);
        let after = chrono::Utc::now().timestamp() as usize;
        assert!(claims.iat >= before && claims.iat <= after);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PARTIE 2 : Tests d'intégration HTTP (axum::test + BDD)
//
// Nécessite DATABASE_URL. Lancer avec : --test-threads=1
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod integration_guards {
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
        Router,
    };
    use tower::ServiceExt;
    use serde_json::{json, Value};

    use crate::auth::claims::Claims;
    use crate::auth::encode_jwt;
    use crate::modules::user::model::UserRole;

    const TEST_SECRET: &str = "test_secret_suffisamment_long_32chars!";

    // ── Helpers ──────────────────────────────────────────────────────────

    async fn build_test_app() -> Router {
        // SAFETY: tests exécutés avec --test-threads=1
        unsafe { std::env::set_var("JWT_SECRET", TEST_SECRET); }

        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL requise pour les tests d'intégration");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(&db_url)
            .await
            .expect("Impossible de se connecter à la BDD de test");

        crate::build_app(pool)
    }

    fn token_for(user_id: i32, email: &str, role: UserRole) -> String {
        // SAFETY: tests exécutés avec --test-threads=1
        unsafe { std::env::set_var("JWT_SECRET", TEST_SECRET); }
        let claims = Claims::new(user_id, email.to_string(), role);
        encode_jwt(&claims).expect("encode_jwt ne devrait pas échouer")
    }

    fn admin_token() -> String {
        token_for(1, "admin@cesizen.com", UserRole::Admin)
    }

    fn user_token(id: i32) -> String {
        token_for(id, &format!("user{}@test.com", id), UserRole::User)
    }

    fn mod_token() -> String {
        token_for(2, "mod@cesizen.com", UserRole::Mod)
    }

    async fn status_of(
        app: Router,
        method: &str,
        uri: &str,
        bearer: Option<&str>,
        body: Option<Value>,
    ) -> StatusCode {
        let mut builder = Request::builder().method(method).uri(uri);

        if let Some(token) = bearer {
            builder = builder.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }

        if body.is_some() {
            builder = builder.header(header::CONTENT_TYPE, "application/json");
        }

        let req_body = match body {
            Some(v) => Body::from(v.to_string()),
            None => Body::empty(),
        };

        let resp = app
            .oneshot(builder.body(req_body).unwrap())
            .await
            .unwrap();

        resp.status()
    }

    async fn assert_rejects_without_token(app: Router, method: &str, uri: &str) {
        let status = status_of(app, method, uri, None, None).await;
        assert!(
            status == StatusCode::BAD_REQUEST || status == StatusCode::UNAUTHORIZED,
            "[{}] {} sans token → attendu 400|401, obtenu {}",
            method, uri, status
        );
    }

    async fn assert_rejects_invalid_token(app: Router, method: &str, uri: &str) {
        let status = status_of(
            app, method, uri,
            Some("token.completement.bidon"),
            None,
        ).await;
        assert!(
            status == StatusCode::BAD_REQUEST || status == StatusCode::UNAUTHORIZED,
            "[{}] {} token invalide → attendu 400|401, obtenu {}",
            method, uri, status
        );
    }

    // =====================================================================
    // A. Routes PUBLIQUES — doivent répondre sans token
    // =====================================================================

    #[tokio::test]
    async fn public_get_article_by_id_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "GET", "/article/1", None, None).await;
        assert_ne!(s, StatusCode::BAD_REQUEST, "GET /article/1 devrait être public");
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_search_article_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "POST", "/article/search", None, Some(json!({}))).await;
        assert_ne!(s, StatusCode::BAD_REQUEST);
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_get_feeling_by_id_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "GET", "/feeling/1", None, None).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_search_feeling_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "POST", "/feeling/search", None, Some(json!({}))).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_get_feeling_category_by_id_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "GET", "/feeling_category/1", None, None).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_search_feeling_category_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "POST", "/feeling_category/search", None, Some(json!({}))).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_get_tag_by_id_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "GET", "/tag/1", None, None).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_get_all_tags_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "GET", "/tag/all", None, None).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_create_user_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(app, "POST", "/user", None, Some(json!({"email": "incomplete"}))).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED, "POST /user (inscription) devrait être public");
    }

    #[tokio::test]
    async fn public_login_no_auth_needed() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/user/login", None,
            Some(json!({"email": "x@x.com", "password": "wrong"})),
        ).await;
        assert_ne!(s, StatusCode::UNAUTHORIZED, "POST /user/login devrait être public");
    }

    // =====================================================================
    // B. Routes ADMIN — RequireAdmin
    // =====================================================================

    // ── Article ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn create_article_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/article").await;
    }

    #[tokio::test]
    async fn create_article_rejects_invalid_token() {
        let app = build_test_app().await;
        assert_rejects_invalid_token(app, "POST", "/article").await;
    }

    #[tokio::test]
    async fn create_article_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/article",
            Some(&user_token(99)),
            Some(json!({"title": "test", "content": "test"})),
        ).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "POST /article avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn create_article_rejects_mod_role() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/article",
            Some(&mod_token()),
            Some(json!({"title": "test", "content": "test"})),
        ).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "POST /article avec rôle Mod → attendu 400|403, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn delete_article_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "DELETE", "/article/1").await;
    }

    #[tokio::test]
    async fn delete_article_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(app, "DELETE", "/article/1", Some(&user_token(99)), None).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "DELETE /article/1 avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    // ── Feeling ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn create_feeling_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/feeling").await;
    }

    #[tokio::test]
    async fn create_feeling_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/feeling",
            Some(&user_token(99)),
            Some(json!({"name": "test"})),
        ).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "POST /feeling avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn update_feeling_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "PUT", "/feeling/1").await;
    }

    #[tokio::test]
    async fn delete_feeling_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "DELETE", "/feeling/1").await;
    }

    // ── Feeling Category ─────────────────────────────────────────────────

    #[tokio::test]
    async fn create_feeling_category_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/feeling_category").await;
    }

    #[tokio::test]
    async fn create_feeling_category_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/feeling_category",
            Some(&user_token(99)),
            Some(json!({"name": "test"})),
        ).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "POST /feeling_category avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn delete_feeling_category_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "DELETE", "/feeling_category/1").await;
    }

    // ── Tag ──────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn create_tag_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/tag").await;
    }

    #[tokio::test]
    async fn create_tag_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/tag",
            Some(&user_token(99)),
            Some(json!({"name": "test"})),
        ).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "POST /tag avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn delete_tag_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "DELETE", "/tag/1").await;
    }

    #[tokio::test]
    async fn delete_tag_rejects_mod_role() {
        let app = build_test_app().await;
        let s = status_of(app, "DELETE", "/tag/1", Some(&mod_token()), None).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "DELETE /tag/1 avec rôle Mod → attendu 400|403, obtenu {}", s
        );
    }

    // ── User search / delete ─────────────────────────────────────────────

    #[tokio::test]
    async fn search_user_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/user/search").await;
    }

    #[tokio::test]
    async fn search_user_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(
            app, "POST", "/user/search",
            Some(&user_token(99)),
            Some(json!({})),
        ).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "POST /user/search avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn delete_user_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "DELETE", "/user/1").await;
    }

    #[tokio::test]
    async fn delete_user_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(app, "DELETE", "/user/1", Some(&user_token(99)), None).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "DELETE /user/1 avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    // ── Feeling Tracker delete ───────────────────────────────────────────

    #[tokio::test]
    async fn delete_feeling_tracker_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "DELETE", "/feeling_tracker/1").await;
    }

    #[tokio::test]
    async fn delete_feeling_tracker_rejects_user_role() {
        let app = build_test_app().await;
        let s = status_of(app, "DELETE", "/feeling_tracker/1", Some(&user_token(99)), None).await;
        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::FORBIDDEN,
            "DELETE /feeling_tracker/1 avec rôle User → attendu 400|403, obtenu {}", s
        );
    }

    // =====================================================================
    // C. Routes AUTH (RequireAuth)
    // =====================================================================

    #[tokio::test]
    async fn get_user_by_id_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "GET", "/user/1").await;
    }

    #[tokio::test]
    async fn get_user_by_id_rejects_invalid_token() {
        let app = build_test_app().await;
        assert_rejects_invalid_token(app, "GET", "/user/1").await;
    }

    #[tokio::test]
    async fn update_user_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "PUT", "/user/1").await;
    }

    #[tokio::test]
    async fn get_feeling_tracker_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "GET", "/feeling_tracker/1").await;
    }

    #[tokio::test]
    async fn create_feeling_tracker_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/feeling_tracker").await;
    }

    #[tokio::test]
    async fn update_feeling_tracker_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "PUT", "/feeling_tracker/1").await;
    }

    #[tokio::test]
    async fn search_feeling_tracker_rejects_without_token() {
        let app = build_test_app().await;
        assert_rejects_without_token(app, "POST", "/feeling_tracker/search").await;
    }

    // =====================================================================
    // D. Ownership
    // =====================================================================

    #[tokio::test]
    async fn user_cannot_get_another_users_profile() {
        let app = build_test_app().await;
        let s = status_of(app, "GET", "/user/1", Some(&user_token(99)), None).await;
        assert!(
            s == StatusCode::FORBIDDEN || s == StatusCode::NOT_FOUND,
            "GET /user/1 par user 99 → attendu 403|404, obtenu {}", s
        );
    }

    #[tokio::test]
    async fn user_cannot_update_another_users_profile() {
        let app = build_test_app().await;
        let s = status_of(
            app, "PUT", "/user/1",
            Some(&user_token(99)),
            Some(json!({"username": "Hacker"})),
        ).await;
        assert!(
            s == StatusCode::FORBIDDEN || s == StatusCode::BAD_REQUEST
            || s == StatusCode::UNPROCESSABLE_ENTITY,
            "PUT /user/1 par user 99 → attendu 403|400, obtenu {}", s
        );
    }

    // =====================================================================
    // E. Header Authorization malformé
    // =====================================================================

    #[tokio::test]
    async fn missing_bearer_prefix_is_rejected() {
        let app = build_test_app().await;

        let resp = app.oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/article/1")
                .header(header::AUTHORIZATION, admin_token())
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert!(
            resp.status() == StatusCode::BAD_REQUEST
                || resp.status() == StatusCode::UNAUTHORIZED,
            "Authorization sans 'Bearer ' → attendu 400|401, obtenu {}",
            resp.status()
        );
    }

    #[tokio::test]
    async fn empty_bearer_token_is_rejected() {
        let app = build_test_app().await;
        let resp = app.oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/article/1")
                .header(header::AUTHORIZATION, "Bearer ")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert!(
            resp.status() == StatusCode::BAD_REQUEST
                || resp.status() == StatusCode::UNAUTHORIZED,
            "Bearer vide → attendu 400|401, obtenu {}",
            resp.status()
        );
    }

    // =====================================================================
    // F. Token expiré
    // =====================================================================

    #[tokio::test]
    async fn expired_token_is_rejected() {
        unsafe { std::env::set_var("JWT_SECRET", TEST_SECRET); }

        let expired_claims = Claims {
            sub: 1,
            email: "expired@test.com".to_string(),
            role: UserRole::Admin,
            iat: 1000,
            exp: 1001,
        };

        let token = encode_jwt(&expired_claims).unwrap();

        let app = build_test_app().await;
        let s = status_of(app, "DELETE", "/article/1", Some(&token), None).await;

        assert!(
            s == StatusCode::BAD_REQUEST || s == StatusCode::UNAUTHORIZED,
            "Token expiré → attendu 400|401, obtenu {}", s
        );
    }
}