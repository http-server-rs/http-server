use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The address to listen on
    #[arg(long)]
    address: String,
    /// The port to listen on
    #[arg(long)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
