use std::path::PathBuf;

use clap::{Parser,Subcommand,ValueEnum,Args};

#[derive(Parser)]
#[command(
    name = "My test app",
    version = "v1.0",
    author = "Sandeep Singh",
    about = "A simple impl for clap crate!!",
    long_about = None,
)]
struct Cli{
    // for -v , --verbose
    #[arg(short,long,action=clap::ArgAction::Count)]
    verbose: u8,

    #[arg(long,default_value="info",env="LOG_LEVEL")]
    log_level : LogLevel,

    #[arg(long,env="MY_CONFIG_FILE")]
    config: Option<PathBuf>,

    #[arg(short,long,num_args=1..,value_hint=clap::ValueHint::FilePath)]
    files: Vec<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum,Clone)]
enum LogLevel{
    Info,Debug,Trace,
}

#[derive(Subcommand)]
enum Commands{
    
    Build {
        #[arg(long)]
        release: bool,

        #[arg(short,long,default_value="4")]
        jobs: u8,
    },
    Run(RunStruct),
    Deploy(DeployStruct),
}

#[derive(Args)]
struct RunStruct{
    #[arg(required=true)]
    executable: String,

    #[arg(last=true,value_name="args")]
    other_args: Vec<String>,
}

#[derive(Args)]
struct DeployStruct{
    #[arg(long,required=true)]
    dry_run: bool,

    #[arg(long,default_value="8080",value_parser=clap::value_parser!(u16).range(1..=65535))]
    port: u16,

    #[arg(long,required=true)]
    target: DeploymentTarget
}

#[derive(ValueEnum,Clone)]
enum DeploymentTarget{
    Staging,Production,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}
