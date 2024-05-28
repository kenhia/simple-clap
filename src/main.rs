use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1.0", author = "Ken Hiatt", about = "playing with clap")]
struct Opts {
    #[clap(short, long, help = "verbose mode (repeat for more)", action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(short, long, help = "Enable debug mode")]
    debug: bool,

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
    if opts.debug {
        println!("Debug mode enabled");
    }
    match opts.verbose_level {
        0 => println!("Verbose mode disabled"),
        1 => println!("Verbose mode enabled"),
        _ => println!("Verbose mode enabled (x{})", opts.verbose_level),
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
