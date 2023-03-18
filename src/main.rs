#[macro_use] extern crate prettytable;
use clap::Parser;
use tokio;
mod yaml;
mod request;
mod command;

#[tokio::main]
async fn main() {
    let args = command::Args::parse();
    match yaml::init_yaml_config() {
        Ok(config_path) => {
            let config = yaml::load_yaml_config(config_path);
            let _ = match config {
                Ok(v) => {
                    let response = request::fetch(args, v).await;
                    match response {
                        Ok(()) => {},
                        Err(e) => {
                            println!("{:#?}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("{:#?}", e);
                }
            };
        },
        Err(e) => {
            println!("dictrs.yaml config error: {:#?}", e);
        }
    }
}