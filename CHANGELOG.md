## Unreleased

<Empty>

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
