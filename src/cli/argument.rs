use clap::Arg;

use super::validator;

pub fn make_arguments() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("host")
            .long("host")
            .short("h")
            .help("Host to bind server")
            .value_name("HOST")
            .takes_value(true)
            .default_value("127.0.0.1")
            .validator(validator::is_valid_host),
        Arg::with_name("port")
            .long("port")
            .short("p")
            .help("Port to bind server")
            .value_name("PORT")
            .takes_value(true)
            .default_value("7878")
            .validator(validator::is_valid_port),
        Arg::with_name("config")
            .long("config")
            .short("c")
            .help("Path to TOML configuration file.\nExample: https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml")
            .value_name("CONFIG"),
        Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Prints output to stdout"),
        Arg::with_name("root_dir")
            .index(1)
            .required(false)
            .help("Directory to server files from")
    ]
}