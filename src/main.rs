use clap::{Command, Arg};
use clap::ColorChoice;

fn main() {
    let matches = Command::new("lsblk")
        .version("0.1.0")
        .author("Ken Hiatt <ken.hiatt@gmail.com>")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("device")
                .help("Device to query")
                .required(true)
                .index(1)
        )
        .get_matches();

    if let Some(device) = matches.get_one::<String>("device") {
        println!("Listing device: {}", device);
    } else {
        println!("Listing all devices");
    }
}