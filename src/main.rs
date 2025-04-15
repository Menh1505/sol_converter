use clap::Parser;
use std::path::Path;

/// A CLI tool for sol_convert
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the Solidity (.sol) file
    #[arg(long)]
    sol: String,

    /// Optional: Enable verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Processing {}", cli.sol);
    }

    // Add your processing logic here
    match &cli.sol {
        s if s.ends_with(".sol") => {
            let path = Path::new(s);
            if path.exists() {
                println!("Found Solidity file: {}", cli.sol);
            } else {
                println!("Error: File does not exist: {}", cli.sol);
                std::process::exit(1);
            }
        }
        _ => {
            println!("Not a Solidity file: {}", cli.sol);
            std::process::exit(1);
        }
    }
}
