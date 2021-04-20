<div>
  <h1 align="center">http-server</h1>
  <h4 align="center">Simple and configurable command-line HTTP server</h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/http-server.svg)](https://crates.io/crates/http-server)
  [![Documentation](https://docs.rs/http-server/badge.svg)](https://docs.rs/http-server)
  ![Build](https://github.com/EstebanBorai/http-server/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/http-server/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/http-server/workflows/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/http-server/workflows/test/badge.svg)

</div>

<div align="center">
  <img src="https://github.com/EstebanBorai/http-server/blob/main/docs/screenshot.png?raw=true" width="600" />
</div>

## Installation

```bash
cargo install http-server
```

Check for the installation to be successful.

```bash
http-server --help
```

## Usage

```
http-server [FLAGS] [OPTIONS] [root-dir]
```

### Flags

Flags are provided without any values. For example:

```
http-server --help
```

Name | Short | Long | Description
--- | --- | --- | ---
Help | N/A | `--help` | Prints help information
Version | `-V` | `--version` | Prints version information
Verbose | `-v` | `--verbose` | Prints output to console

### Options

Options receives a value and have support for default values as well.

```
http-server --host 127.0.0.1
```

Name | Short | Long | Description | Default Value
--- | --- | --- | --- | ---
Host | `-h` | `--host` | Address to bind the server | `127.0.0.1`
Port | `-p` | `--port` | Port to bind the server | `7878`
Configuration File | `-c` | `--config` | Specifies a configuration file. [Example](https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml) | N/A
TLS | N/A | `--tls` | Enable TLS for HTTPS connections. Requires a Certificate and Key. [Reference](#tls-reference) | N/A
TLS Ceritificate | N/A | `--tls-cert` | Path to TLS certificate file. **Depends on `--tls`** | `cert.pem`
TLS Key | N/A | `--tls-key` | Path to TLS key file. **Depends on `--tls`** | `key.rsa`
TLS Key Algorithm | N/A | `--tls-key-tls-key-algorithm` | Algorithm used to generate certificate key. **Depends on `--tls`** | `rsa`

## References

The following are some relevant details on features supported by this HTTP Server
solution that may be of the interest of the user.

### TLS Reference

The TLS solution supported for this HTTP Server is built with [rustls](https://github.com/ctz/rustls)
crate along with [hyper-rustls](https://github.com/ctz/hyper-rustls).

When running with TLS support you will need:

- A certificate
- A RSA Private Key for such certificate

A script to generate certificates and keys is available here [tls-cert.sh](./docs/tls-cert.sh).
This script relies on `openssl`, so make sure you have it installed in your system.

Run `http-server` as follows:

```sh
http-server --tls --tls-cert <PATH TO YOUR CERTIFICATE> --tls-key <PATH TO YOUR KEY> --tls-key-algorithm pkcs8
```

## Release

In order to create a release you must push a Git tag as follows

```sh
git tag -a <version> -m <message>
```

**Example**

```sh
git tag -a  v0.1.0 -m "First release"
```

> Tags must follow semver conventions
> Tags must be prefixed with a lowercase `v` letter.

Then push tags as follows:

```sh
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)
