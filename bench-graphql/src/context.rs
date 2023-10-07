use async_graphql::dataloader::{DataLoader, HashMapCache};
use db::DatabasePool;

pub struct GraphqlContext {
    pub pool: DatabasePool,
    pub loaders: DataLoaders,
    pub token: Option<String>,
}

#[derive()]
pub struct DataLoaders {
    pub users: DataLoader<UserDataLoader, HashMapCache>,
}

impl DataLoaders {
    pub fn new(pool: DatabasePool) -> Self {
        Self {
            users: DataLoader::with_cache(
                UserDataLoader::new(pool.clone()),
                tokio::task::spawn,
                HashMapCache::default(),
            ),
        }
    }
}

macro_rules! impl_data_loader {
    // $fn is a Fn(PoolConnection,&[Id]) -> Result<Vec<T>, Error>
    ($name:ident, $model:ty, $fn:expr) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pool: db::DatabasePool,
        }

        impl $name {
            pub fn new(pool: db::DatabasePool) -> Self {
                Self { pool }
            }
        }

        #[async_trait::async_trait]
        impl async_graphql::dataloader::Loader<db::Id> for $name {
            type Value = $model;
            type Error = crate::errors::DataLoaderError;

            async fn load(
                &self,
                keys: &[db::Id],
            ) -> Result<std::collections::HashMap<db::Id, Self::Value>, Self::Error> {
                // Use `MyLoader` to load data.
                tracing::debug!("Beginning load with keys: {:?}", keys);
                let conn = self.pool.acquire_read_only().await?;
                let query_res = $fn(conn, keys).await?;
                let map = query_res.into_iter().map(|v| (v.id.clone(), v)).collect();
                Ok(map)
            }
        }
    };
}

impl_data_loader!(
    UserDataLoader,
    db::models::user::DbUser,
    db::models::user::DbUser::by_ids
);
