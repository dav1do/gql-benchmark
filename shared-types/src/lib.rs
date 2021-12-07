#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct User {
    pub id: String,
}
