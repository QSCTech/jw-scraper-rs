use crate::JWService;
use config::ConfigError;
use interfacer_http::{Helper, http::Response, ResponseExt};
use interfacer_http_hyper::Client;
use serde::{Deserialize, Serialize};
use crate::resp::{LoginPage, HiddenForm};
use crate::req::LoginBody;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub stu_id: String,
    pub password: String,
    pub jwb_base_url: String,
    pub login_view_state: String,
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
async fn test_login_page() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
            .with_request_initializer(crate::helper::request_initializer)
    );
    let login_page: Response<LoginPage> = service.get_login_page().await?;
    assert_eq!(
        &login_page.body().hidden_form,
        &HiddenForm {
            event_argument: "".into(),
            event_target: "".into(),
            view_state: config.login_view_state,
        }
    );
    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
            .with_request_initializer(crate::helper::request_initializer)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            config.login_view_state.as_str(),
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookie = resp.cookie_map().unwrap().get("ASP.NET_SessionId");
    Ok(())
}
