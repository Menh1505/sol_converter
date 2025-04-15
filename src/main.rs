use clap::Parser;

/// A CLI tool for sol_convert
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input file to process
    #[arg(short, long)]
    input: String,

    /// The output file to write to
    #[arg(short, long)]
    output: String,

    /// Optional: Enable verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Processing {} -> {}", cli.input, cli.output);
    }

    // Add your processing logic here
    println!("Input file: {}", cli.input);
    println!("Output file: {}", cli.output);
}
