<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://avatars.githubusercontent.com/u/122044824?s=400&u=a90857a96069dbb669412b1bbca8ef6757610d9c&v=4" height="120" width="120" />
  </div>
  <h1 align="center">http-server</h1>
  <h4 align="center">Simple and configurable command-line HTTP server</h4>
</div>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/http-server.svg)](https://crates.io/crates/http-server)
[![Documentation](https://docs.rs/http-server/badge.svg)](https://docs.rs/http-server)
![Build](https://github.com/http-server-rs/http-server/workflows/build/badge.svg)
![Clippy](https://github.com/http-server-rs/http-server/workflows/clippy/badge.svg)
![Formatter](https://github.com/http-server-rs/http-server/workflows/fmt/badge.svg)
![Tests](https://github.com/http-server-rs/http-server/workflows/test/badge.svg)
![Benches](https://github.com/http-server-rs/http-server/workflows/bench/badge.svg)

</div>

## Development

### Release Build

Build release binaries with:

```bash
make release
```

Then use the following _alias_ for convenience

```bash
alias htps = './target/release/http-server'
```

## Release

In order to create a release you must push a Git tag as follows

```sh
git tag -a <version> -m <message>
```

**Example**

```sh
git tag -a v0.1.0 -m "First release"
```

> Tags must follow semver conventions.
> Tags must be prefixed with a lowercase `v` letter.

Then push tags as follows:

```sh
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request or
an issue. Just by using this project you're helping it grow. Thank you!

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)

[1]: https://github.com/http-server-rs/http-server
