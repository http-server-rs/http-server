use clap::Arg;

use super::validator;

pub struct CliArgument {
    long: &'static str,
    short: Option<&'static str>,
    help: &'static str,
}

impl CliArgument {
    pub fn name(&self) -> &str {
        self.long
    }
}

pub const HOST: CliArgument = CliArgument {
    long: "host",
    short: Some("h"),
    help: "Host to bind server",
};

pub const PORT: CliArgument = CliArgument {
    long: "port",
    short: Some("p"),
    help: "Port to bind server",
};

pub const CONFIG: CliArgument = CliArgument {
    long: "config",
    short: Some("c"),
    help: "Path to TOML configuration file.\nExample: https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml",
};

pub const VERBOSE: CliArgument = CliArgument {
    long: "verbose",
    short: Some("v"),
    help: "Prints output to stdout",
};

pub const ROOT_DIR: CliArgument = CliArgument {
    long: "root_dir",
    short: None,
    help: "Directory to server files from",
};

pub const TLS: CliArgument = CliArgument {
    long: "tls",
    short: None,
    help: "Enable TLS for HTTPS connections",
};

pub const TLS_CERTIFICATE: CliArgument = CliArgument {
    long: "tls_cert",
    short: None,
    help: "Path to TLS certificate file",
};

pub const TLS_KEY: CliArgument = CliArgument {
    long: "tls_key",
    short: None,
    help: "Path to TLS key file",
};

pub const TLS_KEY_ALGORITHM: CliArgument = CliArgument {
    long: "tls_key_alg",
    short: None,
    help: "TLS key algorithm. Could be either \"rsa\" or \"pkcs8\"",
};

impl<'a> From<CliArgument> for Arg<'a, 'a> {
    fn from(arg: CliArgument) -> Arg<'a, 'a> {
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
