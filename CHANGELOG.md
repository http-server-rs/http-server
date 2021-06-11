## Unreleased

<Empty>

<a name="v0.3.3"></a>
## v0.3.3 (2021-06-10)

> Requires Rust: rustc 1.52.1 (9bc8c42bb 2021-05-09)

#### Improvements

* Implement static file serving addon

Introduce the Addon concept to write application logic out of the
server implementation itself and aim to a modularized architecture.

Implements a static file serving addon to support sending files to
the client. The static file serving addon also provides a scoped
file system abstraction which securely resolve files in the file system
tree.

<a name="v0.3.2"></a>
## v0.3.2 (2021-06-08)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Features

* Add crate logo by @PalyZambrano

#### Improvements

* Avoid clonning server instance by using Arc<Server> over `Server.clone()`

Currently when creating a server instance and spawing a Tokio task
the server instance is cloned (`Server` derive-implements the "Clone" trait).

Instead the same instance is used and a single Arc<Server> is handled to the
concurrent Tokio task.

This reduces repetition and memory consumption and also keeps the Server
instance unique and consistent through threads.

* Resolve static file serving with `resolve_path` instead of `resolve` from
`hyper_staticfile`. This makes file serving more conventional and also
versatile, by relying on the Request URI instead of the whole `hyper::Request`
instance.

* `FileExplorer` middleware now consumes an `Arc<hyper::Request<Body>>` instead
of taking ownership of the `hyper::Request<Body>` instance

* Less transformations are required for a `Request` instance in order to serve
a static file.

* `HttpHandler` now is built over an `Arc<Config>` instead of consuming the
`Config` instance. Previously the `Config` instance uses to implement the `Clone`
trait, now an `Arc<Config>` is used in its place making the `Config` struct
easier to mantain an extensible.

* `MiddlewareAfter` now supports the `hyper::Request<Body>` as well, this brings
support to read from the `hyper::Request<Body>` on middleware after execution.

<a name="v0.3.1"></a>
## v0.3.1 (2021-05-31)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Fixes

* Fix crate binary name to be "http-server" instead of "main"

<a name="v0.3.0"></a>
## v0.3.0 (2021-05-09)

> Requires Rust: rustc 1.49.0 (e1884a8e3 2020-12-29)

#### Features

* A `Middleware` is implemented bringing the capabilities to act on
`hyper::Request<Body>` before executing the main handler (a.k.a File Explorer)
and to act on the `hyper::Response<Body>` after executing the main handler.
This helps implementing future features which relies on acting on different
stages of the HTTP/S request lifecycle such as logging, authentication, caching
and so on.

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
services to programmatically build a server instance from the provided `Config`

* Replace `full` feature flags on `tokio` and `hyper` with sepecific features,
to reduce crate load on compile time. This improve build times and crate size.

* Improved tests coverage by implementing tests for CLI arguments and config
file parsing

#### Fixes

* Fix issue where the `root_dir` is taken _as is_ from the CLI
arguments resulting on `./` instead of the current working
directory for the `FileExplorer`

* Fix issue where loading config without `root_dir` defined panics
and uses the current working directory as default `root_dir` as is
done for CLI

* Fix issue where errors returned by any internal service are not
logged to stderr

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
