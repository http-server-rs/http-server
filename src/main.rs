mod cli;
mod config;

fn main() {
    let cli_app = cli::make_app();

    config::Config::from(cli_app);
}
