use async_graphql::{Context, ErrorExtensions, Object};
use shared_types::User;

use crate::{services::user::UserService, GraphqlContext};
use util::ResultLogger;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn get_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: async_graphql::ID,
    ) -> async_graphql::Result<Option<User>> {
        let gql_ctx = ctx.data::<GraphqlContext>()?;

        // TODO: add auth checks
        // gql_ctx
        //     .subject
        //     .check_admin_membership()
        //     .log_warn()
        //     .map_err(|e| e.extend())?;

        let result = UserService::new(&gql_ctx)
            .by_id(&id.to_string())
            .await
            .log_warn()
            .map_err(|e| e.extend())?;
        Ok(None)
    }
}
