use std::str::FromStr;
use std::sync::Arc;

use http::header::USER_AGENT;
use http::header::{HeaderName, HeaderValue};
use hyper::client::HttpConnector;
use hyper::{Body, Client, Response, Uri};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

use crate::server::middleware::Request;

pub struct Proxy {
    client: Client<HttpsConnector<HttpConnector>>,
    upstream: Uri,
}

impl Proxy {
    pub fn new(upstream: &str) -> Self {
        let https_connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = Client::builder().build::<_, hyper::Body>(https_connector);
        let upstream = Uri::from_str(upstream).unwrap();

        Proxy { client, upstream }
    }

    pub async fn handle(&self, request: Request<Body>) -> Response<Body> {
        self.remove_hbh_headers(Arc::clone(&request)).await;
        self.add_via_header(Arc::clone(&request)).await;

        let mut outogoing = self.map_incoming_request(Arc::clone(&request)).await;
        let outgoing_headers = outogoing.headers_mut();

        // Host must be the authority from the proxied URL
        outgoing_headers.remove(http::header::HOST);
        outgoing_headers.append(
            http::header::HOST,
            HeaderValue::from_str(self.upstream.authority().unwrap().as_str()).unwrap(),
        );

        let client = self.client.clone();

        tokio::spawn(async move { client.request(outogoing).await.unwrap() })
            .await
            .unwrap()
    }

    /// Creates a `Via` HTTP header for the provided HTTP Request.
    ///
    /// The Via general header is added by proxies, both forward and reverse, and
    /// can appear in the request or response headers. It is used for tracking
    /// message forwards, avoiding request loops, and identifying the protocol
    /// capabilities of senders along the request/response chain.
    ///
    /// Via: [ <protocol-name> "/" ] <protocol-version> <host> [ ":" <port> ]
    ///
    /// ## References
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Via
    async fn add_via_header(&self, request: Request<Body>) {
        let mut request = request.lock().await;
        let via_header_str = format!("{:?} Rust http-server", request.version());
        let via_header = HeaderValue::from_str(&via_header_str).unwrap();

        if let Some(current_via_header) = request.headers().get("via") {
            let current_via_header = current_via_header.to_str().unwrap();

            if current_via_header.contains(&via_header_str) {
                return;
            }

            let mut via_set = current_via_header.split(',').collect::<Vec<&str>>();

            via_set.push(&via_header_str);

            let proxies_list = via_set.join(", ");

            request.headers_mut().remove(HeaderName::from_static("via"));
            request.headers_mut().append(
                HeaderName::from_static("via"),
                HeaderValue::from_str(proxies_list.as_str()).unwrap(),
            );
            return;
        }

        request
            .headers_mut()
            .append(HeaderName::from_static("via"), via_header);
    }

    /// Removes Hop-by-Hop headers which are only meaningful for a singles
    /// transport-level connection and should not be stored by caches following
    /// RFC2616.
    ///
    /// The following HTTP/1.1 headers are hop-by-hop headers:
    ///
    ///   - Connection
    ///   - Keep-Alive
    ///   - Proxy-Authenticate
    ///   - Proxy-Authorization
    ///   - TE
    ///   - Trailers
    ///   - Transfer-Encoding
    ///   - Upgrade
    ///
    /// ## References
    ///
    /// http://www.w3.org/Protocols/rfc2616/rfc2616-sec13.html
    async fn remove_hbh_headers(&self, request: Request<Body>) {
        use http::header::{
            CONNECTION, PROXY_AUTHENTICATE, PROXY_AUTHORIZATION, TE, TRAILER, TRANSFER_ENCODING,
            UPGRADE,
        };

        let mut request = request.lock().await;
        let headers = request.headers_mut();

        headers.remove(CONNECTION);
        headers.remove(HeaderName::from_static("keep-alive"));
        headers.remove(PROXY_AUTHENTICATE);
        headers.remove(PROXY_AUTHORIZATION);
        headers.remove(TE);
        headers.remove(TRAILER);
        headers.remove(TRANSFER_ENCODING);
        headers.remove(UPGRADE);
    }

    /// Maps a _incoming_ HTTP request into a _outgoing_ HTTP request.
    async fn map_incoming_request(&self, incoming: Request<Body>) -> hyper::Request<Body> {
        let incoming = incoming.lock().await;
        let upstream_uri = self.map_upstream_uri(incoming.uri());
        let mut request = hyper::Request::builder()
            .uri(upstream_uri)
            .method(incoming.method())
            .body(Body::empty())
            .unwrap();
        let headers = request.headers_mut();

        *headers = incoming.headers().clone();

        // TODO: Instead of append and removing it would be great to support
        // some kind of `set` operation which adds if not present or replaces
        // if present.
        //
        // Host must be the authority from the proxied URL
        headers.remove(http::header::HOST);
        headers.append(
            http::header::HOST,
            HeaderValue::from_str(self.upstream.authority().unwrap().as_str()).unwrap(),
        );

        // Specify Proxy as User Agent
        headers.remove(USER_AGENT).unwrap();
        headers.append(USER_AGENT, HeaderValue::from_static("Rust http-server/1.0"));

        request
    }

    fn map_upstream_uri(&self, incoming_uri: &Uri) -> Uri {
        let scheme = self.upstream.scheme_str().unwrap();
        let authority = self.upstream.authority().unwrap().as_str();
        let path_and_query = if let Some(paq) = incoming_uri.path_and_query() {
            paq.as_str()
        } else {
            ""
        };

        Uri::builder()
            .scheme(scheme)
            .authority(authority)
            .path_and_query(path_and_query)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use http::header::{HeaderName, HeaderValue};
    use http::header::{
        CONNECTION, PROXY_AUTHENTICATE, PROXY_AUTHORIZATION, TE, TRAILER, TRANSFER_ENCODING,
        UPGRADE,
    };
    use hyper::Body;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    use super::Proxy;

    #[tokio::test]
    async fn adds_via_header_if_not_present() {
        let proxy = Proxy::new("https://example.com");
        let request = http::Request::new(Body::empty());
        let request = Arc::new(Mutex::new(request));

        proxy.add_via_header(Arc::clone(&request)).await;

        let request = request.lock().await;
        let headers = request.headers();

        assert!(headers.get(&HeaderName::from_static("via")).is_some());

        let via_header_value = headers.get(&HeaderName::from_static("via")).unwrap();
        let via_header_value = via_header_value.to_str().unwrap();

        assert_eq!(via_header_value, "HTTP/1.1 Rust http-server");
    }

    #[tokio::test]
    async fn appends_via_header_if_another_is_present() {
        let proxy = Proxy::new("https://example.com");
        let mut request = http::Request::new(Body::empty());
        let headers = request.headers_mut();

        headers.append(
            &HeaderName::from_static("via"),
            HeaderValue::from_str("HTTP/1.1 GoodProxy").unwrap(),
        );

        let request = Arc::new(Mutex::new(request));

        proxy.add_via_header(Arc::clone(&request)).await;

        let request = request.lock().await;
        let headers = request.headers();

        assert!(headers.get(&HeaderName::from_static("via")).is_some());

        let via_header_value = headers.get(&HeaderName::from_static("via")).unwrap();
        let via_header_value = via_header_value.to_str().unwrap();

        assert_eq!(
            via_header_value,
            "HTTP/1.1 GoodProxy, HTTP/1.1 Rust http-server"
        );
    }

    #[tokio::test]
    async fn removes_hbh_headers() {
        let proxy = Proxy::new("https://example.com");
        let mut request = http::Request::new(Body::empty());
        let headers = request.headers_mut();
        let headers_to_add = vec![
            (CONNECTION, HeaderValue::from_str("keep-alive").unwrap()),
            (
                PROXY_AUTHENTICATE,
                HeaderValue::from_static(r#"Basic realm="Access to the internal site""#),
            ),
            (
                PROXY_AUTHORIZATION,
                HeaderValue::from_static("Basic YWxhZGRpbjpvcGVuc2VzYW1l"),
            ),
            (TE, HeaderValue::from_static("compress")),
            (TRAILER, HeaderValue::from_static("Expires")),
            (TRANSFER_ENCODING, HeaderValue::from_static("chunked")),
            (UPGRADE, HeaderValue::from_static("example/1, foo/2")),
        ];

        for (name, value) in headers_to_add {
            headers.append(name, value);
        }

        let request = Arc::new(Mutex::new(request));

        proxy.remove_hbh_headers(Arc::clone(&request)).await;

        let request = request.lock().await;

        assert!(!request.headers().contains_key(CONNECTION));
        assert!(!request.headers().contains_key(PROXY_AUTHENTICATE));
        assert!(!request.headers().contains_key(PROXY_AUTHORIZATION));
        assert!(!request.headers().contains_key(TE));
        assert!(!request.headers().contains_key(TRAILER));
        assert!(!request.headers().contains_key(TRANSFER_ENCODING));
        assert!(!request.headers().contains_key(UPGRADE));
    }
}
