use clap::Parser;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    author = "Ken Hiatt",
    about = "playing with clap",
)]
struct Opts {
    #[clap(short, long, help = "verbose mode")]
    verbose: bool,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[command(name="info", about = "get info about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(index = 1, help = "device to get info about")]
    device: String,
}

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Info(info) => {
            if opts.verbose {
                println!("Verbose mode enabled");
            }
            println!("Getting info about device: {}", info.device);
        }
    }
}