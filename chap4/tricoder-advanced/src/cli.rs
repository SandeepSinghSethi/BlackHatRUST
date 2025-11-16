use clap::{Parser,ValueEnum,Subcommand,Args};


#[derive(Parser)]
#[command(
    name= "Tricoder [->][^][<-]",
    version="v1.0.0",
    author="test developer",
    about="A simple async port scanner tool to simplify the process of initial reconnaisance.",
    long_about= None,
)]
pub struct Cli{
    #[arg(short,long,action=clap::ArgAction::Count)]
    verbose: u8,

    #[arg(long,default_value="info",env="LOG_LEVEL")]
    log_level : LogLevel,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    #[command(name="modules",about="List all modules")]
    Modules(ModuleStruct),

    #[command(name="scan",about="Scans the given target")]
    Scan(ScanStruct),
}

#[derive(Args)]
pub struct ModuleStruct{
    //left for - if any args are required...
}

#[derive(Args)]
pub struct ScanStruct{
    #[arg(short,long,required=true)]
    target: String,
}

#[derive(ValueEnum,Clone)]
pub enum LogLevel {
    INFO,DEBUG,ERROR,
}

// functions....
pub async fn modules()  {
    log::debug!("In the modules func.");
}

pub async fn scan(target: ScanStruct) {
    log::debug!("In the scan func.");
    println!("Target: {}",target.target);
}

