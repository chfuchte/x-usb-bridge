use std::net::SocketAddr;

use anyhow::{Ok, Result};
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::web::assets;

pub(crate) async fn run_web_server(port: u16) -> Result<()> {
    let app = Router::new().fallback(assets::static_handler);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await?;
    info!("Web server up and running: http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
