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
http-server [FLAGS] [OPTIONS] [root_dir]
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

## Release

In order to create a release you must push a Git tag as follows

```shell
git tag -a <version> -m <message>
```

**Example**

```shell
git tag -a  v0.1.0 -m "First release"
```

> Tags must follow semver conventions
> Tags must be prefixed with a lowercase `v` letter.

Then push tags as follows:

```shell
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in http-server by you, shall be dual licensed as above, without any additional
terms or conditions.
