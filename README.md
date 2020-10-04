<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/EstebanBorai/http-server/main/docs/http-logo.png" height="120" width="200" />
  </div>
  <h1 align="center">http-server</h1>
  <h4 align="center">Zero-configuration command-line HTTP server</h4>
</div>

<div align="center">

  ![Build](https://github.com/EstebanBorai/http-server/workflows/build/badge.svg)
  ![Lint](https://github.com/EstebanBorai/http-server/workflows/clippy/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/http-server/workflows/tests/badge.svg)

</div>

## Modules

This project is composed by 3 main modules:

### The `cli` module

Which is in charge of gathering command-line arguments and options

### The `config` module

Defines the shape of the HTTP server configuration. This module acts on an input
(the `cli` for instance), to build an instance of `Config`, which is then passed to
the `server` module

### The `server` module

The main logic for the _HTTP Server_. This module must receive a `Config` instance
which is used to build an _HTTP Server_ instance, and then binds the server process
to the specified socket address.

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Licensed under the MIT License

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in http-server by you, shall be dual licensed as above, without any additional
terms or conditions.
