use crate::{Error, GraphqlContext};
use db::{models::user::User as DbUser, Id};
use std::str::FromStr;
use util::ResultLogger;

pub struct UserService<'ctx> {
    ctx: &'ctx GraphqlContext,
}

impl<'ctx> UserService<'ctx> {
    pub fn new(ctx: &'ctx GraphqlContext) -> Self {
        Self { ctx }
    }
}

impl UserService<'_> {
    pub async fn by_id(&self, id: &str) -> Result<Option<DbUser>, Error> {
        let uuid = Id::from_str(id).log_warn().map_err(|e| Error::BadInput {
            field: "id".into(),
            message: format!("{}", e),
        })?;
        let user = DbUser::by_id(&self.ctx.pool, &uuid).await?;
        Ok(user)
    }
}
