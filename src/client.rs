use interfacer_http::{url::Url, Helper};
use interfacer_http_hyper::hyper::{self, client::connect::Connect, client::HttpConnector, Body};
use interfacer_http_hyper::Client;

/// client with default `HttpConnector`
///
/// ```rust
/// use zju_jw_scraper::client::client;
/// let client = client("http://github.com".parse().unwrap());
/// ```
pub fn client(base_url: Url) -> Client<HttpConnector> {
    client_on(base_url, HttpConnector::new())
}

/// client with custom `Connect`
///
/// ```rust
/// use zju_jw_scraper::client::client_on;
/// use hyper_tls::HttpsConnector;
/// let client = client_on("http://github.com".parse().unwrap(), HttpsConnector::new().unwrap());
/// ```
pub fn client_on<C>(base_url: Url, connector: C) -> Client<C>
where
    C: Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    let base_client = hyper::Client::builder().build::<_, Body>(connector);
    Client::base_on(base_client).with_helper(
        Helper::new()
            .with_base_url(base_url)
            .with_mime_matcher(crate::helper::mime_matcher),
    )
}
