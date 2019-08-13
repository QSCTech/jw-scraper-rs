#![feature(custom_attribute, async_await, param_attrs)]
#![allow(unused_attributes)]

pub mod req;
pub mod resp;

use interfacer_http::{
    content_types::{APPLICATION_FORM, TEXT_HTML_CHARSET_GB2312},
    http::header::COOKIE,
    http_interface, Response, Result,
};
use req::LoginBody;
use resp::{CoursesPage, LoginPage};

#[http_interface]
trait JWInterface: Clone {
    #[get("default2.aspx")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_login_page(&self) -> Result<Response<LoginPage>>;

    #[post("default2.aspx", APPLICATION_FORM)]
    #[expect(302)]
    async fn login<'a>(&self, #[body] body: &LoginBody<'a>) -> Result<Response<()>>;

    #[get("xskbcx.aspx?xh={stu_id}")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_default_courses(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: &str
    ) -> Result<Response<CoursesPage>>;
}
