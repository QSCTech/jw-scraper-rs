const BASE_URL: &str = "http://jwbinfosys.zju.edu.cn";

use interfacer_http::{ async_trait, http::{header::CONTENT_TYPE, Request, Response, Version}, url::Url, HttpClient, Helper};

use interfacer_http_hyper::{Result, Error};

use crate::{resp::{LoginPage, HiddenForm, tests::LOGIN_PAGE}, JWService};
use encoding::label::encoding_from_whatwg_label;
use encoding::EncoderTrap;
use tokio::prelude::*;

pub struct Client<F> {
    helper: Helper,
    handler: fn(Request<Vec<u8>>) -> F,
}

impl<F> Client<F>
where
    F: Future<Output = Result<Response<Vec<u8>>>> + Send + 'static,
{
    pub fn new(base_url: Url, handler: fn(Request<Vec<u8>>) -> F) -> Self {
        Self {
            handler,
            helper: Helper::new()
                .with_base_url(base_url)
                .with_request_initializer(crate::helper::request_initializer),
        }
    }
}

#[async_trait]
impl<F> HttpClient for Client<F>
where
    F: Future<Output = Result<Response<Vec<u8>>>> + Send + 'static
{
    type Err = Error;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
        (self.handler)(req).await
    }

    fn helper(&self) -> &Helper {
        &self.helper
    }
}

async fn login_page_handler(req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
    assert_eq!(
        req.uri(),
        Url::parse(BASE_URL)?.join("default2.aspx")?.as_str()
    );
    Ok(Response::builder()
        .status(200)
        .version(Version::HTTP_11)
        .header(CONTENT_TYPE, "text/html; charset=gb2312")
        .body(
            encoding_from_whatwg_label("gb2312")
                .unwrap()
                .encode(LOGIN_PAGE, EncoderTrap::Strict)
                .unwrap(),
        )?
    )
}

#[tokio::test]
async fn test_login_page() -> Result<()> {
    let service = Client::new(BASE_URL.parse()?, login_page_handler);
    let page = service.get_login_page().await?;
    assert_eq!(
        &page.body().hidden_form,
        &HiddenForm {
            event_argument: "".into(),
            event_target: "".into(),
            view_state: "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=".into(),
        }
    );
    Ok(())
}
