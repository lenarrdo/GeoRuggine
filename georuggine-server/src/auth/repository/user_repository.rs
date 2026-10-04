use crate::auth::domain::user::User;
use sqlx::SqlitePool;

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(
        db_pool: &SqlitePool,
        username: &str,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?);")
            .bind(username)
            .bind(password_hash)
            .execute(db_pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_username(
        db_pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(db_pool)
        .await?;

        Ok(user)
    }
}
