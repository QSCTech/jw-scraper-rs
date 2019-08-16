#![feature(custom_attribute, async_await, param_attrs)]
#![allow(unused_attributes)]

pub mod req;
pub mod resp;

use interfacer_http::{
    content_types::{APPLICATION_FORM_CHARSET_GB2312, TEXT_HTML_CHARSET_GB2312},
    http::header::COOKIE,
    http_interface, Response, Result,
};
use req::{CoursesReq, ExamsReq, LoginBody, ScoresReq};
use resp::{
    CourseInfo, CoursesPage, ExamsPage, LoginPage, MajorScoresPage, ScoresBasePage, ScoresPage,
    TotalCreditPage,
};

#[http_interface]
trait JWInterface: Clone {
    #[get("default2.aspx")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_login_page(&self) -> Result<Response<LoginPage>>;

    #[post("default2.aspx", APPLICATION_FORM_CHARSET_GB2312)]
    #[expect(302)]
    async fn login<'a>(&self, #[body] body: &LoginBody<'a>) -> Result<Response<()>>;

    #[get("html_kc/{code}.html")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_course_info(&self, code: &str) -> Result<Response<CourseInfo>>;

    #[get("xskbcx.aspx?xh={stu_id}")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_default_courses(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<CoursesPage>>;

    #[post("xskbcx.aspx?xh={stu_id}", APPLICATION_FORM_CHARSET_GB2312)]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_courses<'a>(
        &self,
        stu_id: &str,
        #[body] body: &CoursesReq<'a>,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<CoursesPage>>;

    #[get("xskscx.aspx?xh={stu_id}")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_default_exams(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<ExamsPage>>;

    #[post("xskscx.aspx?xh={stu_id}", APPLICATION_FORM_CHARSET_GB2312)]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_exams<'a>(
        &self,
        stu_id: &str,
        #[body] body: &ExamsReq<'a>,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<ExamsPage>>;

    #[get("xscj.aspx?xh={stu_id}")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_scores_base(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<ScoresBasePage>>;

    #[post("xscj.aspx?xh={stu_id}", APPLICATION_FORM_CHARSET_GB2312)]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_scores<'a>(
        &self,
        stu_id: &str,
        #[body] body: &ScoresReq<'a>,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<ScoresPage>>;

    #[get("xscj_zg.aspx?xh={stu_id}")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_major_scores(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<MajorScoresPage>>;

    #[get("xs_txsqddy.aspx?xh={stu_id}")]
    #[expect(200, TEXT_HTML_CHARSET_GB2312)]
    async fn get_total_credit(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<TotalCreditPage>>;
}
