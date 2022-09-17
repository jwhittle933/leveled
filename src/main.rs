pub mod grpc;
pub mod level;
pub mod routing;

pub mod proto {
    tonic::include_proto!("levelservice");
}

use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use grpc::LevelServiceImpl;
use proto::level_service_server::LevelServiceServer;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        // .with(tracing_subscriber::EnvFilter::new(
        //     std::env::var("RUST_LOG").unwrap_or_else(|_| "RUST_LOG=debug".into()),
        // ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let json_api = Router::new()
        // `GET /` goes to `root`
        .route("/", get(web_root))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    let grpc_api = LevelServiceServer::new(LevelServiceImpl::default());
    let service = routing::multiplex::MultiplexService::new(json_api, grpc_api);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();
}

async fn web_root() -> &'static str {
    "Hello, World!"
}
