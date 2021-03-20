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
    ]
}
