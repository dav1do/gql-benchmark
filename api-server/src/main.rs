use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api_server::{graphql_route, health, Error};
use std::net::SocketAddr;

use tracing_subscriber::{prelude::*, EnvFilter};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    if let Ok(p) = dotenv::dotenv() {
        println!(
            r#"{{ "message": "Loading environment from {}"}}"#,
            p.display()
        );
    }

    tracing_subscriber::fmt()
        .json() //removing this makes much nicer terminal logs locally
        .with_env_filter(EnvFilter::from_default_env())
        .finish()
        .init();

    let listen_addr = std::env::var("SERVER_ADDR").unwrap_or("0.0.0.0:80".into());
    let listen_addr: SocketAddr = listen_addr.parse().unwrap();

    let ctx = api_server::prepare_context().await?;
    let schema = bench_graphql::new_schema().finish();

    HttpServer::new(move || {
        let scope = actix_web::web::scope("/api")
            .service(health)
            .service(graphql_route);
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(ctx.clone()))
            .wrap(Logger::new(
                "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(scope)
    })
    .bind(listen_addr)?
    .run()
    .await?;

    Ok(())
}
