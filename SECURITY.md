# Security Policy

This command-line HTTP Server is intended to be as configurable as possible,
thus security concerns may vary based on the configuration provided at runtime.

### Data Security

If you are running this server to send/receive sensitive data, please make sure
you configure TLS support and provide a certificate.

See [TLS/HTTPS configuration][1] for more details.

### Host Security

This software is primary written in Rust. Rust is well known for its security
principles. By design, access to invalid memory addresses or invalid references
are mostly eliminated. Even though these warranties many not apply in some
use cases (those where `unsafe` code is introduced), this software tends to
avoid the usage of these code blocks. If you find an opportunity to eliminate
`unsafe` code and you are not sure how to apply it, please don't hesitate to
contact the software editors by opening an issue or a pull request.

## Reporting a Vulnerability

If you have found any security issue using this software, please don't
hesitate to send an email to estebanborai@gmail.com.

[1]: https://github.com/EstebanBorai/http-server#tls-https
