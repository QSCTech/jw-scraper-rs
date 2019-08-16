const BASE_URL: &str = "http://jwbinfosys.zju.edu.cn";

use interfacer_http::{
    content_types::{CHARSET_GB2312, TEXT_HTML_CHARSET_GB2312},
    http::{header::CONTENT_TYPE, Request, Response, Version},
    url::Url,
    HttpClient, HttpService, RequestFail,
};

use crate::{resp::{LoginPage, HiddenForm, tests::LOGIN_PAGE}, JWInterface};
use encoding::label::encoding_from_whatwg_label;
use encoding::EncoderTrap;
use tokio::prelude::*;

pub struct Service<T: HttpClient> {
    client: T,
    base_url: Url,
}

impl<T, F> Service<T>
where
    F: Future<Output = Result<Response<Vec<u8>>, RequestFail>> + Send + 'static,
    T: Fn(Request<Vec<u8>>) -> F + Sync,
{
    pub fn new(base_url: Url, client: T) -> Self {
        Self { client, base_url }
    }
}

impl<T: HttpClient> HttpService for Service<T> {
    type Client = T;

    fn get_base_url(&self) -> &Url {
        &self.base_url
    }

    fn get_client(&self) -> &Self::Client {
        &self.client
    }
}

#[tokio::test]
async fn test_login_page() -> Result<(), failure::Error> {
    let service = Service::new(BASE_URL.parse()?, async move |req| {
        assert_eq!(
            req.uri(),
            Url::parse(BASE_URL)?.join("default2.aspx")?.as_str()
        );
        Ok(Response::builder()
            .status(200)
            .version(Version::HTTP_11)
            .header(CONTENT_TYPE, TEXT_HTML_CHARSET_GB2312)
            .body(
                encoding_from_whatwg_label(CHARSET_GB2312)
                    .unwrap()
                    .encode(LOGIN_PAGE, EncoderTrap::Strict)
                    .unwrap(),
            )
            .unwrap())
    });
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
