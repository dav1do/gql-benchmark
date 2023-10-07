use crate::{DatabasePool, DateTime, Error, Id};
use sqlx::{pool::PoolConnection, query_as, FromRow, Postgres};
use util::ResultLogger;

#[derive(Debug, Clone, FromRow)]
pub struct DbUser {
    pub id: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

impl DbUser {
    pub async fn by_id(pool: &DatabasePool, id: &Id) -> Result<Option<DbUser>, Error> {
        let mut conn = pool.acquire().await.log_warn()?;

        let user = query_as!(
            DbUser,
            "SELECT * from bench_user where id = $1 and deleted_at is null",
            id,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(user)
    }

    pub async fn by_ids(
        mut conn: PoolConnection<Postgres>,
        ids: &[Id],
    ) -> Result<Vec<DbUser>, Error> {
        let users = query_as!(
            DbUser,
            "SELECT * from bench_user where id = ANY($1) and deleted_at is null",
            ids,
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(users)
    }
}

impl From<DbUser> for shared_types::User {
    fn from(value: DbUser) -> Self {
        Self {
            id: value.id.to_string(),
        }
    }
}
