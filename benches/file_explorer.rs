use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use hyper::client::HttpConnector;
use hyper::Client;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
    static ref HTTP_CLIENT: Client<HttpConnector> = Client::new();
}

async fn http_get(uri: &str) {
    HTTP_CLIENT.get(uri.parse().unwrap()).await.unwrap();
}

fn get_root(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("get_root", |b| {
        b.to_async(&rt).iter(|| http_get("http://127.0.0.1:7878"));
    });
}

fn get_file(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("get_file", |b| {
        b.to_async(&rt)
            .iter(|| http_get("http://127.0.0.1:7878/docs/screenshot.png"));
    });
}

fn not_found_file(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("not_found_file", |b| {
        b.to_async(&rt)
            .iter(|| http_get("http://127.0.0.1:7878/thisfiledoesntexists123"));
    });
}

criterion_group!(benches, get_root, get_file, not_found_file);
criterion_main!(benches);
