use hyper_tls::HttpsConnector;
use zju_jw_scraper::{helper::mime_matcher, Body, Client, Helper, HyperClient, JWService};

// replace std_id and password to yours, otherwise this example fails
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_client = HyperClient::builder().build::<_, Body>(HttpsConnector::new()?);
    let service = Client::base_on(base_client).with_helper(
        Helper::new()
            .with_base_url("https://jw.zjuqsc.com".parse()?)
            .with_mime_matcher(mime_matcher),
    );
    let cookie = service.login("3190000000", "test").await?;
    assert!(!cookie.is_empty());
    Ok(())
}
