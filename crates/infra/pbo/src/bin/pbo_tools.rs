use env_logger;
use log::error;
use clap::Parser;
use arma3_pbo::cli::args::Cli;
use arma3_pbo::cli::CliProcessor;
use arma3_pbo::core::constants::DEFAULT_TIMEOUT;

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
