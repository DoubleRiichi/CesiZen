use sqlx::PgPool;
use crate::errors::app::AppError;
use crate::modules::user::dto::{UserCreate, UserGet, UserSearchParams, UserUpdate};
use crate::modules::user::model::{UserRole, UserRow};

pub struct UserRepository;

impl UserRepository {

    pub async fn index(pool: &PgPool) -> Result<Vec<UserRow>, AppError> {
        let users = sqlx::query_as::<_, UserRow>(
            r#"SELECT * from "user""#
        )
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn by_mail(pool: &PgPool, email: &str) -> Result<UserRow, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM "user" WHERE email = $1"#)
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn by_id(pool: &PgPool, id: i32) -> Result<UserRow, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM "user" WHERE id = $1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    // ✅ Après
    pub async fn create(pool: &PgPool, user: UserCreate, role: &UserRole, hashed: &str) -> Result<UserRow, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"INSERT INTO "user"
        (username, email, password, role, age, avatar, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4::user_role, $5, $6, false, NOW(), NOW())
        RETURNING *"#)
            .bind(user.username)  // $1 username
            .bind(user.email)     // $2 email
            .bind(hashed)         // $3 password hashé ✅
            .bind(role)           // $4 role
            .bind(user.age)       // $5 age
            .bind(user.avatar)    // $6 avatar ✅
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn update(pool: &PgPool, id: i32, user: UserUpdate, hashed: &str) -> Result<UserRow, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"UPDATE "user" SET
                  email = $1,
                  password = $2,
                  is_active = $3,
                  updated_at = NOW(),
                  avatar = $4
                  WHERE id = $5
            RETURNING *"#
        )
            .bind(user.email)
            .bind(hashed)
            .bind(user.is_active)
            .bind(user.avatar)
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn change_role(pool: &PgPool, user_id: i32, role: &UserRole) -> Result<UserRow, AppError> {
        let user = sqlx::query_as::<_, UserRow>(

            r#"UPDATE "user" SET
                  role = $1::user_role,
                  updated_at = NOW()
                  WHERE id = $2
                  RETURNING *"#,
            // user.role,
            // user_id,
        )
            .bind(role)
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"DELETE FROM "user" WHERE id = $1"#
            // id
        ).bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }



    pub async fn search(
        pool: &PgPool,
        params: UserSearchParams,
        page_size: i32,
    ) -> Result<Vec<UserRow>, AppError> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"SELECT * FROM "user" WHERE 1=1"#,
        );

        if let Some(username) = params.username {
            qb.push(" AND username ILIKE ");
            qb.push_bind(format!("%{}%", username));
        }

        if let Some(email) = params.email {
            qb.push(" AND email ILIKE ");
            qb.push_bind(format!("%{}%", email));
        }

        if let Some(avatar) = params.avatar {
            qb.push(" AND avatar = ");
            qb.push_bind(avatar);
        }

        if let Some(age) = params.age {
            qb.push(" AND age >= ");
            qb.push_bind(age);
        }

        if let Some(is_active) = params.is_active {
            qb.push(" AND is_active = ");
            qb.push_bind(is_active);
        }

        if let Some(role) = params.role {
            qb.push(" AND role = ");
            qb.push_bind(role);
        }

        if let Some(start) = params.start_at {
            qb.push(" AND created_at >= ");  // ✅ pas de préfixe "a."
            qb.push_bind(start);
        }

        if let Some(end) = params.end_at {
            qb.push(" AND created_at <= ");  // ✅ corrigé
            qb.push_bind(end);
        }

        if let Some(cursor) = params.cursor {
            qb.push(" AND created_at < ");   // ✅ corrigé
            qb.push_bind(cursor);
        }

        qb.push(" ORDER BY created_at DESC LIMIT ");  // ✅ GROUP BY supprimé
        qb.push_bind(page_size);

        let query = qb.build_query_as::<UserRow>();
        Ok(query.fetch_all(pool).await?)
    }
}