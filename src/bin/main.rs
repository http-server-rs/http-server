use http_server::cli;
use http_server::config::Config;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::build();
    let matches = cli.get_matches();
    let config = Config::try_from(matches)?;

    println!("{:?}", config);
    Ok(())
}
