use batch_scanner::batch_process::{self, Args};
use clap::Parser;

fn main() -> std::io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Run the batch parser using the new batch_process module
    batch_process::run(args).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}
