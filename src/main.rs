use std::fs::OpenOptions;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1.0", author = "Ken Hiatt", about = "playing with clap")]
struct Opts {
    #[clap(short, long, env="PLAY_VERBOSE", help = "verbose mode (repeat for more)", action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(long, help = "Log to file")]
    log_file: bool,

    #[clap(short, long, env = "PLAY_DEBUG", help = "Enable debug mode")]
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

    let mut log_builder = env_logger::Builder::new();
    log_builder.filter_level(match opts.verbose_level {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    });

    if opts.log_file {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("play.log")
            .expect("Failed to open log file");
        log_builder.target(env_logger::Target::Pipe(Box::new(log_file)));
    }
    log_builder.init();

    if opts.debug {
        println!("Debug mode enabled");
    }

    log::trace!("trace message logging is enabled");
    log::debug!("debug message logging is enabled");
    log::info!("info message logging is enabled");
    log::warn!("warn message logging is enabled");

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
