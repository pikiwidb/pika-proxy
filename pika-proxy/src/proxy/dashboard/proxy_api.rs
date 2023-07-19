use axum::{routing::get, Router};
use std::{io::Result, sync::Arc};

use crate::proxy::config::Config;

// Proxy 表示 pika-proxy 的唯一元数据, 应该是作为全局静态的
async fn server_proxy_api(proxy: Arc<Config>) -> Result<()> {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
