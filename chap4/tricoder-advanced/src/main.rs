use std::env;
use clap::Parser;

mod error;
pub use error::Error;
mod common_ports;
mod cli;
use cli::Commands;
mod dns;
mod ports;
mod modules;


#[tokio::main(flavor="multi_thread")]
#[warn(unused_variables)]
async fn main() {
    unsafe {
        env::set_var("RUST_LOG", "info");
    };
    env_logger::init();

    let cli = cli::Cli::parse();

    match cli.commands{
        Commands::Modules(_) =>{
            cli::modules();
        }

        Commands::Scan(scan_args) =>{
            cli::scan(&scan_args.target).await;
        }
    }
    log::info!("This is informational , working fine!");    
}
