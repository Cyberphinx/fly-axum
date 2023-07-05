use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello, I am a test for deploying Rust Axum Backend in fly.io!" }),
        )
        .route(
            "/spiders",
            get(|| async {
                "Incy wincy spider climbed up the waterspout,\nDown came the rain and washed the spider out."}),
        );

    // Binding to all addresses
    // This listens to all addresses on both IPv4 and IPv6, on port 8080.
    // It's important to do this because your app would otherwise need to know about the 172.19.0.0/16 IP it should bind to specifically.
    // Binding to IPv6 is not required, but is likely a good idea going forward.
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
