## Unreleased

<Empty>

<a name="v0.3.0"></a>
## v0.3.0

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Features

* Support for Cross-Origin Resource Sharing

* Using the `--cors` flag when running the HTTP Server will now provide a
CORS configuration which allows requests from any origin

* The server configuration file supports a fully configurable CORS field now

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

#### Improvements

* Codebase refactor for `server` module, introducing middleware support for
services to programatically build a server instance from the provided `Config`

* Replace `full` feature flags on `tokio` and `hyper` with sepecific features,
to reduce crate load on compile time. This improve build times and crate size.

#### Fixes

* Fix issue where the `root_dir` is taken _as is_ from the CLI
arguments resulting on `./` instead of the current working
directory for the `FileExplorer`

* Fix issue where loading config without `root_dir` defined panics
and uses the current working directory as default `root_dir` as is
done for CLI

<a name="v0.2.2"></a>
## v0.2.2 (2021-04-22)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Fixes

* Fix issue where the root_path is taken _as is_ from the CLI
arguments resulting on `./` instead of the current working
directory for the `FileExplorer`

<a name="v0.2.1"></a>
## v0.2.1 (2021-04-22)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Fixes

* Fix issue where `FileExplorer` entry link is not prefixed with `/` on some
paths causing the link to be invalid.

<a name="v0.2.0"></a>
## v0.2.0 (2021-04-19)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Features

* Add support for HTTPS serving using TLS
  * Support TLS certificates
  * Support TLS keys using RSA or PKCS8 algorithm

#### Improvements

* Refactor CLI implementation to use _structopt_

<a name="v0.1.0"></a>
## v0.1.0 (2021-04-02)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Breaking Changes

* Replace `-a`, `--address` with `-h` or `--host` for host address
* Replace `-h` with `--help` for Help

#### Features

* Add support for loading configuration from TOML file by specifying `--config [FILE]`.
  An example of this file is available here [Example](https://github.com/EstebanBorai/http-server/blob/main/fixtures/config.toml)

* File Explorer with rich UI

#### Improvements

* Use `hyper-rs` server for backend
* Serve static files using `hyper_staticfile`
* Upgrade `tokio-rs` to version 1

#### Fixes

* Caching issue caused because HTTP response headers doesn't include `ETag`
