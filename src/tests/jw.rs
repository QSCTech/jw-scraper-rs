use crate::JWInterface;
use config::ConfigError;
use interfacer_http::HttpService;
use interfacer_http_hyper::{Client, Service};
use serde::{Deserialize, Serialize};
use tokio::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub stu_id: String,
    pub password: String,
    pub jwb_base_url: String,
}

impl Config {
    pub fn parse() -> Result<Self, ConfigError> {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name("test-settings").required(false))?
            .merge(config::Environment::with_prefix("TEST"))?;
        settings.try_into::<Self>()
    }
}

#[tokio::test]
async fn test_login() -> Result<(), failure::Error> {
    let config = Config::parse()?;
    let service = Service::new(config.jwb_base_url.parse()?);
    //    let login_page = service.get_login_page().await?;
    Ok(())
}
