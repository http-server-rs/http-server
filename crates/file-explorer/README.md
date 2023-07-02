# file-explorer
A static file explorer for HTTP Server RS

## Usage

```rust
use axum::{http::Request, routing::get, Router};
use file_explorer::FileExplorer;

let file_explorer = FileExplorer::new(opts.root_dir);
let app = Router::new().nest_service("/", file_explorer);
```
