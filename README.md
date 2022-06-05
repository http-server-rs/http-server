<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="./assets/logo.svg" height="120" width="120" />
  </div>
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
  ![Benchs](https://github.com/EstebanBorai/http-server/workflows/bench/badge.svg)

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

Expect the following output

```
USAGE:
    http-server [FLAGS] [OPTIONS] [root-dir]

FLAGS:
        --cors                 Enable Cross-Origin Resource Sharing allowing any origin
        --graceful-shutdown    Waits for all requests to fulfill before shutting down the server
        --gzip                 Enable GZip compression for HTTP Responses
        --help                 Prints help information
        --logger               Prints HTTP request and response details to stdout
        --tls                  Enables HTTPS serving using TLS
    -V, --version              Prints version information
    -v, --verbose              Turns on stdout/stderr logging

OPTIONS:
    -c, --config <config>                          Path to TOML configuration file
    -h, --host <host>                              Host (IP) to bind the server [default: 127.0.0.1]
        --password <password>                      Specifies password for basic authentication
    -p, --port <port>                              Port to bind the server [default: 7878]
        --proxy <proxy>                            Proxy requests to the provided URL
        --tls-cert <tls-cert>                      Path to the TLS Certificate [default: cert.pem]
        --tls-key <tls-key>                        Path to the TLS Key [default: key.rsa]
        --tls-key-algorithm <tls-key-algorithm>    Algorithm used to generate certificate key [default: rsa]
        --username <username>                      Specifies username for basic authentication

ARGS:
    <root-dir>    Directory to serve files from [default: ./]
```

> If you find this output is out-of-date, don't hesitate to open a [PR here][1].

## Configuration

When running the server with no options or flags provided, a set of default
configurations will be set, you can always change this behavior by either
creating your own config with the [Configuration TOML](https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml) file
or by providing CLI arguments described in the [usage](#usage) section.

Name | Description | Default
--- | --- | ---
Host | Address to bind the server | `127.0.0.1`
Port | Port to bind the server | `7878`
Root Directory | The directory to serve files from | `CWD`
File Explorer UI | A File Explorer UI for the directory configured as the _Root Directory_ | Enabled
Configuration File | Specifies a configuration file. [Example](https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml) | Disabled
HTTPS (TLS) | HTTPS Secure connection configuration. Refer to [TLS (HTTPS)](https://github.com/EstebanBorai/http-server#tls-https) reference | Disabled
CORS | Cross-Origin-Resource-Sharing headers support. Refer to [CORS](https://github.com/EstebanBorai/http-server#cross-origin-resource-sharing-cors) reference | Disabled
Compression | GZip compression for HTTP Response Bodies. Refer to [Compression](https://github.com/EstebanBorai/http-server#compression) reference | Disabled
Verbose | Print server details when running. This doesn't include any logging capabilities. | Disabled
Basic Authentication | Authorize requests using Basic Authentication. Refer to [Basic Authentication](https://github.com/EstebanBorai/http-server#basic-authentication)  | Disabled
Logger | Prints HTTP request and response details to stdout | Disabled

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
Cross-Origin Resource Sharing | N/A | `--cors` | Enable Cross-Origin Resource Sharing allowing any origin
GZip Compression | N/A | `--gzip` | Enable GZip compression for responses
Graceful Shutdown | N/A | `--graceful-shutdown` | Waits for all requests to fulfill before shutting down the server
Help | N/A | `--help` | Prints help information
Logger | N/A | `--logger` | Prints HTTP request and response details to stdout
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
TLS Key Algorithm | N/A | `--tls-key-algorithm` | Algorithm used to generate certificate key. **Depends on `--tls`** | `rsa`
Username | N/A | `--username` | Specify the username to validate using basic authentication | N/A
Password | N/A | `--password` | Specify the password to validate using basic authentication. **Depends on `--username`** | N/A
Proxy | N/A | `--proxy` | Proxy requests to the provided URL | N/A

## Request Handlers

This HTTP Proxy supports different _Request Handlers_, this determines how each
incoming HTTP request must be handled, they can't be combinable so you must
choose one based on your needs.

- [File Server](#file-server-handler)
- [Proxy](#proxy-handler)

### File Server Handler

Useful for serving files in the provided directory. Navigation is scoped to the
specified directory, if no directory is provided the CWD will be used.

> This is the default behavior for the HTTP server.

### Proxy Handler

Proxies requests to the provided URL. The URL provided is used as the base URL
for incoming requests.

## References

The following are some relevant details on features supported by this HTTP Server
solution that may be of the interest of the user.

### Compression

Even when compression is supported, by default the server will not compress any
HTTP response contents.

You must specify the compression configuration you want to use, as of today
the server only supports compression with the GZip algorithm, but `brotli` is
also planed to be supported, that's why theres two ways to configure this
server to use compression.

The following MIME types will be skipped from compression:

- `application/gzip`
- `application/octet-stream`
- `application/wasm`
- `application/zip`
- `image/*`
- `video/*`

#### The Configuration File's Compression Section

As suppport for other compression algorithms is planned to be provided in the
future, the configuration file already supports compression settings.

```toml
[compression]
gzip = true
```

#### The `--gzip` flag

Provide the `--gzip` argument to the server when executing it.

```bash
http-server --gzip
```

### TLS (HTTPS)

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

### Cross-Origin Resource Sharing (CORS)

This HTTP Server brings support to CORS headers _out of the box_.
Based on the headers you want to provide to your HTTP Responses, 2
different methods for CORS configuration are available.

By providing the `--cors` option to the `http-server`, CORS headers
will be appended to every HTTP Response, allowing any origin.

For more complex configurations, like specifying an origin, a set of allowed
HTTP methods and more, you should specify the configuration via the configuration
TOML file.

The following example shows all the options available, these options are
mapped to the server configuration during initialization.

```toml
[cors]
allow_credentials = false
allow_headers = ["content-type", "authorization", "content-length"]
allow_methods = ["GET", "PATCH", "POST", "PUT", "DELETE"]
allow_origin = "example.com"
expose_headers = ["*", "authorization"]
max_age = 600
request_headers = ["x-app-version"]
request_method = "GET"
```

### Basic Authentication

Basic Authentication is supported to deny requests when credentials are invalid.
You must provide the allowed `username` and `password` either by using the CLI
options `--username` along with the desired username and `--password` along with
the desired password, or by specifying such values through the configuration
TOML file.

```toml
[basic_auth]
username = "John"
password = "Appleseed"
```

### Proxy

The HTTP Server is able to proxy requests to an specified URL.

By using the proxy the FileExplorer wont be available, the proxy is considered
a _Request Handler_.

The config TOML file can be used to provide proxy configurations:

```toml
[proxy]
url = "https://example.com"
```

## Roadmap

The following roadmap list features to provide for the version `v1.0.0`.

This roadmap still open for suggestions, if you find that theres a missing
feature in this list, you would like to work on or expect for the first
stable release, please contact software editors by opening an issue or a
discussion.

If you want to contribute to one of these, please make sure
theres an issue tracking the feature and ping me. Otherwise
open an issue to be assigned and track the progress there.

- [x] Logging
  - [x] Request/Response Logging
  - [x] Service Config Loggins
- [ ] File Explorer
  - [x] Modified Date
  - [x] File Size
  - [ ] Breadcrumb Navigation
  - [ ] File Upload
  - [ ] Filtering
  - [ ] Sorting
    - [ ] Sort By: File Name
    - [ ] Sort By: File Size
    - [ ] Sort By: File Modified Date
    - [x] Directories First
    - [ ] Files First
- [x] HTTPS/TLS Serving
  - [x] HTTPS/TLS Support
- [ ] Compression
  - [x] `gzip/deflate` Compression
  - [ ] `brotli` Compression
- [ ] CORS
  - [x] Cross Origin Resource Sharing
    - [x] Allow Credentials
    - [x] Allow Headers
    - [x] Allow Methods
    - [x] Allow Origin
    - [x] Expose Headers
    - [x] Max Age
    - [x] Request Headers
    - [x] Request Methods
    - [ ] Multiple Origins (#8)
- [ ] Cache Control
  - [ ] `Last-Modified` and `ETag`
  - [ ] Respond with 304 to `If-Modified-Since`
- [ ] Partial Request
  - [ ] `Accept-Ranges`
  - [ ] `Content-Range`
  - [ ] `If-Range`
  - [ ] `If-Match`
  - [ ] `Range`
- [x] Standalone Builds
  - [x] macOS
  - [x] Linux
  - [x] Windows
- [ ] Development Server
  - [ ] Live Reload
- [x] Proxy
  - [x] URL Configuration
- [x] Basic Authentication
  - [x] Username
  - [x] Password
- [x] Graceful Shutdown

## Release

In order to create a release you must push a Git tag as follows

```sh
git tag -a <version> -m <message>
```

**Example**

```sh
git tag -a v0.1.0 -m "First release"
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

[1]: https://github.com/EstebanBorai/http-server
