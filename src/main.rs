pub mod models;
pub mod proxy;
pub mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("config/proxy.toml"))]
    config: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}
