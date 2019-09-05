use interfacer_http::{url::Url, Helper};
use interfacer_http_hyper::hyper::{self, client::connect::Connect, client::HttpConnector, Body};
use interfacer_http_hyper::Client;

pub fn client(base_url: Url) -> Client<HttpConnector> {
    client_on(base_url, HttpConnector::new())
}

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
