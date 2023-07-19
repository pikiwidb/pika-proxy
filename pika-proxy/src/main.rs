use clap::Parser;

use proxy::server::{ProxyOptions, ProxyServer};

mod error;
mod models;
mod proxy;
mod utils;

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
        .worker_threads(num_cpus::get()) // cpu*2个工作线程
        .enable_io() // 可在runtime中使用异步IO
        .enable_time() // 可在runtime中使用异步计时器(timer)
        .build() // 创建runtime
        .unwrap();
    let mut proxy = ProxyServer::new(&option).expect("server config error");
    rt.block_on(proxy.serve_proxy())
        .expect("unhandled fatal error");
}
