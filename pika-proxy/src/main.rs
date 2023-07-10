pub mod error;
pub mod models;
pub mod proxy;
pub mod utils;

use clap::Parser;
use proxy::proxy::{Proxy, ProxyOptions};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("config/proxy.toml"))]
    config_path: String,
}

fn main() {
    let args = Args::parse();
    // let config: Config = Config::from_path(args.config_path.as_str());
    let option = ProxyOptions {
        config_path: args.config_path,
    };
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8) // 8个工作线程
        .enable_io() // 可在runtime中使用异步IO
        .enable_time() // 可在runtime中使用异步计时器(timer)
        .build() // 创建runtime
        .unwrap();

    let proxy = Proxy::new(&option).unwrap();

    rt.block_on(proxy.serve_proxy());
}
