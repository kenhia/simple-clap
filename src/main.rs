use clap::Parser;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    author = "Ken Hiatt",
    about = "playing with clap",
)]
struct Opts {
    #[clap(help="Device to query")]
    device: String,
}

fn main() {
    let opts = Opts::parse();
    println!("device: {}", opts.device);
}