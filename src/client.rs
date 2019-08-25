use interfacer_http_hyper::{Client, hyper::client::HttpConnector};
use interfacer_http::{Helper, url::Url};

pub fn client(base_url: Url) -> Client<HttpConnector> {
    Client::new().with_helper(
        Helper::new()
            .with_base_url(base_url)
            .with_mime_matcher(crate::helper::mime_matcher)
    )
}