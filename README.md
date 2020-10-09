<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/EstebanBorai/http-server/main/docs/http-logo.png" height="120" width="200" />
  </div>
  <h1 align="center">http-server</h1>
  <h4 align="center">Command-line HTTP Server</h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/http-server.svg)](https://crates.io/crates/http-server)
  ![Build](https://github.com/EstebanBorai/http-server/workflows/build/badge.svg)
  ![Lint](https://github.com/EstebanBorai/http-server/workflows/clippy/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/http-server/workflows/tests/badge.svg)

</div>

## Index

- [Installation](#installation)
- [Usage](#usage)
    - [Flags](#flags)
    - [Options](#options)
- [Contributing](#contributing)
- [License](#license)
    - [Contribution](#contribution)

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
Help | `h` | `help` | Prints help information
Version | `V` | `version` | Prints version information

### Options

Options are provided with a value and also have default values. For example:

```
http-server --address 127.0.0.1
```

Name | Short | Long | Description | Default Value
--- | --- | --- | --- | ---
Address | `a` | `address` | Address to bind the server | `0.0.0.0`
Port | `p` | `port` | Port to bind the server | `7878`

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Licensed under the MIT License

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in http-server by you, shall be dual licensed as above, without any additional
terms or conditions.
