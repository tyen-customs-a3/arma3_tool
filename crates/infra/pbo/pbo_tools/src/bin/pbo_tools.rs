use env_logger;
use log::error;
use clap::Parser;
use pbo_tools::cli::args::Cli;
use pbo_tools::cli::CliProcessor;
use pbo_tools::core::constants::DEFAULT_TIMEOUT;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let cli = Cli::parse();
    let processor = CliProcessor::new(cli.timeout);
    
    if let Err(e) = processor.process_command(cli.command).await {
        error!("{}", e);
        std::process::exit(1);
    }
}
