use crate::Error;

use bench_graphql::GraphqlContext;
use tracing::info;

pub struct Context {
    pub pool: db::DatabasePool,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl Context {
    /// not actually authed rn
    pub async fn as_authed_graphql_context(
        self,
        token: Option<String>,
    ) -> Result<GraphqlContext, Error> {
        Ok(GraphqlContext { pool: self.pool })
    }
}

pub async fn prepare_context() -> Result<Context, Error> {
    info!("Preparing api context");
    let term = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let _s_int = signal_hook::flag::register(signal_hook::consts::SIGINT, term.clone())?;
    let _s_term = signal_hook::flag::register(signal_hook::consts::SIGTERM, term.clone())?;

    let db_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL");
    let pool = db::DatabasePool::new_from_config(&db_url, &None).await?;

    Ok(Context { pool })
}
