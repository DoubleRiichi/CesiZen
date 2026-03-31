//
// #[cfg(test)]mod unit {
//     use crate::auth::{encode_jwt, decode_jwt};
//     use crate::auth::claims::Claims;
//     use crate::modules::user::model::UserRole;
//
//     fn test_claims() -> Claims {
//         Claims::new(42, "test@example.com".to_string(), UserRole::User)
//     }
//
//     fn admin_claims() -> Claims {
//         Claims::new(1, "admin@example.com".to_string(), UserRole::Admin)
//     }
//
//     fn mod_claims() -> Claims {
//         Claims::new(2, "mod@example.com".to_string(), UserRole::Mod)
//     }
//
//     // ── encode_jwt / decode_jwt ───────────────────────────────
//
//     #[test]
//     fn encode_then_decode_roundtrip() {
//         std::env::set_var("JWT_SECRET", "test_secret_suffisamment_long_32chars");
//
//         let claims = test_claims();
//         let token = encode_jwt(&claims).expect("encode should succeed");
//         assert!(!token.is_empty());
//
//         let decoded = decode_jwt(&token).expect("decode should succeed");
//         assert_eq!(decoded.sub, 42);
//         assert_eq!(decoded.email, "test@example.com");
//         assert!(matches!(decoded.role, UserRole::User));
//     }
//
//     #[test]
//     fn decode_invalid_token_returns_error() {
//         std::env::set_var("JWT_SECRET", "test_secret_suffisamment_long_32chars");
//
//         let result = decode_jwt("ceci.n_est.pas_un_token");
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn decode_tampered_token_returns_error() {
//         std::env::set_var("JWT_SECRET", "test_secret_suffisamment_long_32chars");
//
//         let token = encode_jwt(&test_claims()).unwrap();
//         // Tamper le payload
//         let parts: Vec<&str> = token.split('.').collect();
//         let tampered = format!("{}.TAMPERED.{}", parts[0], parts[2]);
//         assert!(decode_jwt(&tampered).is_err());
//     }
//
//     #[test]
//     fn decode_token_wrong_secret_returns_error() {
//         std::env::set_var("JWT_SECRET", "secret_A_suffisamment_long_pour_jwt");
//         let token = encode_jwt(&test_claims()).unwrap();
//
//         std::env::set_var("JWT_SECRET", "secret_B_different_suffisamment_long");
//         assert!(decode_jwt(&token).is_err());
//
//         // Rétablir pour ne pas polluer les autres tests
//         std::env::set_var("JWT_SECRET", "test_secret_suffisamment_long_32chars");
//     }
//
//     #[test]
//     fn claims_new_sets_correct_fields() {
//         let claims = Claims::new(99, "user@test.com".to_string(), UserRole::Admin);
//         assert_eq!(claims.sub, 99);
//         assert_eq!(claims.email, "user@test.com");
//         assert!(matches!(claims.role, UserRole::Admin));
//         assert!(claims.exp > claims.iat);
//     }
//
//     #[test]
//     fn claims_expiration_is_24h_in_future() {
//         let before = chrono::Utc::now().timestamp() as usize;
//         let claims = Claims::new(1, "x@y.com".to_string(), UserRole::User);
//         let after = chrono::Utc::now().timestamp() as usize;
//
//         // exp doit être ~24h après now
//         let min_exp = before + 23 * 3600;
//         let max_exp = after + 25 * 3600;
//         assert!(claims.exp >= min_exp, "exp trop tôt");
//         assert!(claims.exp <= max_exp, "exp trop tard");
//     }
//
//
//     #[test]
//     fn admin_role_display() {
//         assert_eq!(UserRole::Admin.to_string(), "Admin");
//     }
//
//     #[test]
//     fn mod_role_display() {
//         assert_eq!(UserRole::Mod.to_string(), "Mod");
//     }
//
//     #[test]
//     fn user_role_display() {
//         assert_eq!(UserRole::User.to_string(), "User");
//     }
//
//     #[test]
//     fn claims_with_all_roles_encode_correctly() {
//         std::env::set_var("JWT_SECRET", "test_secret_suffisamment_long_32chars");
//
//         for (id, role) in [(1, UserRole::Admin), (2, UserRole::Mod), (3, UserRole::User)] {
//             let claims = Claims::new(id, format!("{}@test.com", id), role);
//             let token = encode_jwt(&claims).unwrap();
//             let decoded = decode_jwt(&token).unwrap();
//             assert_eq!(decoded.sub, id);
//         }
//     }
// }
//
//
// #[cfg(test)]
// #[cfg(feature = "integration")]
// mod http {
//     use axum::{body::Body, http::{Request, StatusCode}, Router};
//     use serde_json::{json, Value};
//     use tower::ServiceExt;
//     use crate::tests::integration_tests::helpers::build_test_app;
//
//     async fn body_json(response: axum::response::Response) -> Value {
//         let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
//             .await
//             .unwrap();
//         serde_json::from_slice(&bytes).unwrap_or(Value::Null)
//     }
//
//     async fn create_test_user(app: &Router) -> (i64, String, String) {
//         let email = format!("auth_test_{}@example.com", uuid::Uuid::new_v4());
//         let password = "ValidPass123".to_string();
//
//         let body = json!({
//             "username": format!("auth_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
//             "password": password,
//             "email": email,
//             "avatar": "https://avatar.com/auth",
//             "age": 25
//         });
//
//         let resp = app.clone().oneshot(
//             Request::builder()
//                 .method("POST").uri("/user/")
//                 .header("Content-Type", "application/json")
//                 .body(Body::from(body.to_string())).unwrap(),
//         ).await.unwrap();
//
//         assert_eq!(resp.status(), StatusCode::OK, "Impossible de créer l'utilisateur de test");
//         let user: Value = body_json(resp).await;
//         (user["id"].as_i64().unwrap(), email, password)
//     }
//
//     async fn delete_test_user(app: &Router, id: i64) {
//         let _ = app.clone().oneshot(
//             Request::builder()
//                 .method("DELETE").uri(format!("/user/{}", id))
//                 .body(Body::empty()).unwrap(),
//         ).await;
//     }
//
//
//     #[tokio::test]
//     async fn login_valid_credentials_returns_token() {
//         let app: Router = build_test_app().await;
//         let (user_id, email, password) = create_test_user(&app).await;
//
//         let body = json!({ "email": email, "password": password });
//
//         let resp = app.clone().oneshot(
//             Request::builder()
//                 .method("POST").uri("/user/login")
//                 .header("Content-Type", "application/json")
//                 .body(Body::from(body.to_string())).unwrap(),
//         ).await.unwrap();
//
//         assert_eq!(resp.status(), StatusCode::OK);
//         let json: Value = body_json(resp).await;
//
//         let token = json["token"].as_str().expect("token manquant dans la réponse");
//         assert!(!token.is_empty());
//         // Le token doit avoir 3 parties séparées par des points (header.payload.signature)
//         assert_eq!(token.split('.').count(), 3, "format JWT invalide");
//
//         assert_eq!(json["user"]["id"].as_i64().unwrap(), user_id);
//         assert_eq!(json["user"]["email"].as_str().unwrap(), email);
//
//         delete_test_user(&app, user_id).await;
//     }
//
//     #[tokio::test]
//     async fn login_wrong_password_returns_400() {
//         let app: Router = build_test_app().await;
//         let (user_id, email, _) = create_test_user(&app).await;
//
//         let body = json!({ "email": email, "password": "wrong_password" });
//
//         let resp = app.clone().oneshot(
//             Request::builder()
//                 .method("POST").uri("/user/login")
//                 .header("Content-Type", "application/json")
//                 .body(Body::from(body.to_string())).unwrap(),
//         ).await.unwrap();
//
//         assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//
//         delete_test_user(&app, user_id).await;
//     }
//
//     #[tokio::test]
//     async fn login_unknown_email_returns_400() {
//         let app: Router = build_test_app().await;
//
//         let body = json!({
//             "email": "nobody@nowhere.com",
//             "password": "whatever123"
//         });
//
//         let resp = app.oneshot(
//             Request::builder()
//                 .method("POST").uri("/user/login")
//                 .header("Content-Type", "application/json")
//                 .body(Body::from(body.to_string())).unwrap(),
//         ).await.unwrap();
//
//         assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//     }
//
//     #[tokio::test]
//     async fn login_missing_fields_returns_422() {
//         let app: Router = build_test_app().await;
//
//         let body = json!({ "email": "test@test.com" }); // password manquant
//
//         let resp = app.oneshot(
//             Request::builder()
//                 .method("POST").uri("/user/login")
//                 .header("Content-Type", "application/json")
//                 .body(Body::from(body.to_string())).unwrap(),
//         ).await.unwrap();
//
//         assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
//     }
//
//
//     #[tokio::test]
//     async fn protected_route_without_token_returns_400() {
//         let app: Router = build_test_app().await;
//
//         let resp = app.oneshot(
//             Request::builder()
//                 .method("DELETE").uri("/user/1")
//                 .body(Body::empty()).unwrap(),
//         ).await.unwrap();
//
//         // Sans Authorization header → 400 Bad Request (AppError::Validation)
//         assert!(
//             resp.status() == StatusCode::BAD_REQUEST
//                 || resp.status() == StatusCode::UNAUTHORIZED,
//             "Attendu 400 ou 401, obtenu {}",
//             resp.status()
//         );
//     }
//
//     #[tokio::test]
//     async fn protected_route_with_invalid_token_returns_400() {
//         let app: Router = build_test_app().await;
//
//         let resp = app.oneshot(
//             Request::builder()
//                 .method("DELETE").uri("/user/1")
//                 .header("Authorization", "Bearer ceci.n_est.pas.un.token.valide")
//                 .body(Body::empty()).unwrap(),
//         ).await.unwrap();
//
//         assert!(
//             resp.status() == StatusCode::BAD_REQUEST
//                 || resp.status() == StatusCode::UNAUTHORIZED,
//             "Attendu 400 ou 401, obtenu {}",
//             resp.status()
//         );
//     }
//
//     #[tokio::test]
//     async fn protected_route_with_valid_token_is_accessible() {
//         let app: Router = build_test_app().await;
//         let (user_id, email, password) = create_test_user(&app).await;
//
//         let login_body = json!({ "email": email, "password": password });
//         let resp = app.clone().oneshot(
//             Request::builder()
//                 .method("POST").uri("/user/login")
//                 .header("Content-Type", "application/json")
//                 .body(Body::from(login_body.to_string())).unwrap(),
//         ).await.unwrap();
//         let json: Value = body_json(resp).await;
//         let token = json["token"].as_str().unwrap().to_string();
//
//         let resp = app.clone().oneshot(
//             Request::builder()
//                 .method("GET").uri(format!("/user/{}", user_id))
//                 .header("Authorization", format!("Bearer {}", token))
//                 .body(Body::empty()).unwrap(),
//         ).await.unwrap();
//
//         assert_eq!(resp.status(), StatusCode::OK);
//
//         delete_test_user(&app, user_id).await;
//     }
// }