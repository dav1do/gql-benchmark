use crate::{DatabasePool, DateTime, Error, Id};
use sqlx::{query_as, FromRow};
use util::ResultLogger;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

impl User {
    pub async fn by_id(pool: &DatabasePool, id: &Id) -> Result<Option<User>, Error> {
        let mut conn = pool.acquire().await.log_warn()?;

        let user = query_as!(
            User,
            "SELECT * from bench_user where id = $1 and deleted_at is null",
            id,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(user)
    }
}
