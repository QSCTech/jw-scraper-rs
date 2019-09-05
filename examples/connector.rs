use hyper_tls::HttpsConnector;
use zju_jw_scraper::{client::client_on, JWService};

// replace std_id and password to yours, otherwise this example fails
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = client_on("https://jw.zjuqsc.com".parse()?, HttpsConnector::new()?);
    let cookie = service.login("3190000000", "test").await?;
    assert!(!cookie.is_empty());
    Ok(())
}
