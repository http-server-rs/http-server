use clap::Arg;

use super::validator;

pub struct CliArgument<'a> {
    long: &'a str,
    short: Option<&'a str>,
    help: &'a str,
}

impl<'a> CliArgument<'a> {
    pub fn name(&self) -> &'a str {
        self.long
    }
}

pub const HOST: CliArgument<'static> = CliArgument {
    long: "host",
    short: Some("h"),
    help: "Host to bind server",
};

pub const PORT: CliArgument<'static> = CliArgument {
    long: "port",
    short: Some("p"),
    help: "Port to bind server",
};

pub const CONFIG: CliArgument<'static> = CliArgument {
    long: "config",
    short: Some("c"),
    help: "Path to TOML configuration file.\nExample: https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml",
};

pub const VERBOSE: CliArgument<'static> = CliArgument {
    long: "verbose",
    short: Some("v"),
    help: "Prints output to stdout",
};

pub const ROOT_DIR: CliArgument<'static> = CliArgument {
    long: "root_dir",
    short: None,
    help: "Directory to server files from",
};

pub const TLS: CliArgument<'static> = CliArgument {
    long: "tls",
    short: None,
    help: "Enable TLS for HTTPS connections",
};

pub const TLS_CERTIFICATE: CliArgument<'static> = CliArgument {
    long: "tls_cert",
    short: None,
    help: "Path to TLS certificate file",
};

pub const TLS_KEY: CliArgument<'static> = CliArgument {
    long: "tls_key",
    short: None,
    help: "Path to TLS key file",
};

pub const TLS_KEY_ALGORITHM: CliArgument<'static> = CliArgument {
    long: "tls_key_alg",
    short: None,
    help: "TLS key algorithm. Could be either \"rsa\" or \"pkcs8\"",
};

impl<'a> From<CliArgument<'a>> for Arg<'a, 'a> {
    fn from(arg: CliArgument<'a>) -> Arg<'a, 'a> {
        if arg.short.is_some() {
            return Arg::with_name(arg.long)
                .long(arg.long)
                .short(arg.short.unwrap())
                .help(arg.help);
        }

        Arg::with_name(arg.long).long(arg.long).help(arg.help)
    }
}

pub fn make_arguments() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::from(HOST)
            .takes_value(true)
            .default_value("127.0.0.1")
            .validator(validator::is_valid_host),
        Arg::from(PORT)
            .takes_value(true)
            .default_value("7878")
            .validator(validator::is_valid_port),
        Arg::from(CONFIG),
        Arg::from(VERBOSE),
        Arg::from(ROOT_DIR).index(1).required(false),
        Arg::from(TLS).required(false),
        Arg::from(TLS_CERTIFICATE).default_value("cert.pem"),
        Arg::from(TLS_KEY).default_value("key.rsa"),
        Arg::from(TLS_KEY_ALGORITHM)
            .default_value("rsa")
            .validator(validator::is_valid_tls_key_alg),
    ]
}
