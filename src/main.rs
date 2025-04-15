use clap::Parser;

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
    println!("Solidity file: {}", cli.sol);
}
