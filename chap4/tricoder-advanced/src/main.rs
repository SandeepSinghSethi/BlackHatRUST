use std::env;
use clap::Parser;

mod error;
mod common_ports;
mod cli;
use cli::Commands;


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
            cli::modules().await;
        }

        Commands::Scan(scan_args) =>{
            cli::scan(scan_args).await;
        }
    }
    log::info!("This is informational , working fine!");    
}
