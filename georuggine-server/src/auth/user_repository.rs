use sqlx::SqlitePool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserEntity {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
}

pub struct UserRepository {
    db_pool: SqlitePool,
}

impl UserRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        UserRepository { db_pool }
    }

    pub async fn create_user(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<u32, sqlx::Error> {
        let user_id: u32 = sqlx::query_scalar(
            "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id",
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(user_id)
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            "SELECT id, username, password_hash FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(user)
    }

    pub async fn get_users(&self) -> Result<Vec<UserEntity>, sqlx::Error> {
        let users =
            sqlx::query_as::<_, UserEntity>("SELECT id, username, password_hash FROM users")
                .fetch_all(&self.db_pool)
                .await?;

        Ok(users)
    }
}
