use crate::directory::Directory;
use crate::handler::{IntoResponse, Router};
use crate::server::HttpServer;
use bytes::Bytes;
use futures::FutureExt;
use http::Request;
use std::borrow::Cow;
use std::net::SocketAddr;

mod codec;
mod directory;
mod handler;
mod server;

fn main() -> anyhow::Result<()> {
    start()
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let directory = Directory::new("/", "fixtures").await?;

    let router = Router::builder()
        .get("/hello", hello_world)
        .post("/hello", hello_world)
        .merge(directory);

    tracing::info!(?router, "Router");
    tracing::info!("Http server listening on {}", addr);
    HttpServer::default()
        .serve(
            addr,
            router,
            tokio::signal::ctrl_c().map(|_| tracing::info!("Shutting down...")),
        )
        .await?;
    Ok(())
}

async fn hello_world(req: Request<Bytes>) -> impl IntoResponse {
    tracing::info!(?req, "Received request");
    format!(
        "Hello, World! (using {} method), body={}",
        req.method(),
        std::str::from_utf8(req.body().as_ref())
            .map(Cow::Borrowed)
            .unwrap_or_else(|_| Cow::Owned(format!("<{} bytes binary>", req.body().len())))
    )
}
