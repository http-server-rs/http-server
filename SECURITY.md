# Security Policy

This Command-Line HTTP Server is intended to be as configurable as possible,
thus security concerns may vary based on the configuration provided on runtime.

### Data Security

If you are running this server to send/receive sensitive data, please make sure
you configure TLS support and provide a certificate.

Follow on [TLS/HTTPS configuration][1] for more details.

### Host Security

This software is primary written in Rust. Rust is well known for its security
principles by design, access to invalid memory addresses or invalid references
are mostly reduced. Even though these warranties many not apply in some
use cases (those where `unsafe` code is introduced), this software tends to
avoid the usage of these code blocks. If you find any space for replacement
of `unsafe` code and you are not sure on how could you apply it, please don't
hesitate on contacting software editors by opening an issue or a pull request.

## Reporting a Vulnerability

If you have found any issue using this solution, please don't hesitate to
open a Pull Request or send an email to estebanborai@gmail.com if you consider
the issue of urgent priority.

[1]: https://github.com/EstebanBorai/http-server#tls-https
