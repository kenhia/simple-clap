use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1.0", author = "Ken Hiatt", about = "playing with clap")]
struct Opts {
    #[clap(short, long, help = "verbose mode")]
    verbose: bool,
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get info about a device
    Info { device: String },
    /// List devices
    List,
}

fn main() {
    let opts = Opts::parse();
    if opts.verbose {
        println!("Verbose mode enabled");
    }
    match opts.cmd {
        Commands::Info { device } => {
            println!("Getting info about device: {}", device);
        }
        Commands::List => {
            println!("Listing devices");
        }
    }
}
