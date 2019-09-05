use interfacer_http::{url::Url, Helper};
use interfacer_http_hyper::hyper::client::HttpConnector;
pub use interfacer_http_hyper::Client;

pub fn client(base_url: Url) -> Client<HttpConnector> {
    Client::new().with_helper(
        Helper::new()
            .with_base_url(base_url)
            .with_mime_matcher(crate::helper::mime_matcher),
    )
}
