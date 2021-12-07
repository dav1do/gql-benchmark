use async_graphql::Subscription;

// see the examples for how to access context
// https://github.com/async-graphql/examples/blob/be1508f163311f0ca9ed987c9487fd7d28fac942/models/books/src/lib.rs#L97
type HelloWorldStream = std::pin::Pin<Box<dyn futures::Stream<Item = String> + Send>>;

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn hello_world(&self) -> HelloWorldStream {
        let stream = futures::stream::once(async { "Hello World!".to_string() });
        Box::pin(stream)
    }
}
