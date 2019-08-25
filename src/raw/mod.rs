pub mod req;
pub mod resp;

#[cfg(all(test, feature = "test"))]
mod tests;

use interfacer_http::{
    http::header::COOKIE, http::Response,
    http_service,
};
use req::{CoursesReq, ExamsReq, LoginBody, ScoresReq};
use resp::{
    CourseInfo, CoursesPage, ExamsPage, LoginPage, MajorScoresPage, ScoresBasePage, ScoresPage,
    TotalCreditPage,
};

const JWB_COOKIE_NAME: &str = "ASP.NET_SessionId";

#[http_service]
pub trait JWService {
    type Err;

    #[get("default2.aspx")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_login_page(&self) -> Result<Response<LoginPage>, Self::Err>;

    #[post("default2.aspx", "application/x-www-form-urlencoded; charset=gb2312")]
    #[expect(302, "text/html; charset=gb2312")]
    async fn login<'a>(&self, #[body] body: LoginBody<'a>) -> Result<Response<()>, Self::Err>;

    #[get("html_kc/{code}.html")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_course_info(&self, code: &str) -> Result<Response<CourseInfo>, Self::Err>;

    #[get("xskbcx.aspx?xh={stu_id}")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_default_courses(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<CoursesPage>, Self::Err>;

    #[post("xskbcx.aspx?xh={stu_id}", "application/x-www-form-urlencoded; charset=gb2312")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_courses<'a>(
        &self,
        stu_id: &str,
        #[body] body: CoursesReq<'a>,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<CoursesPage>, Self::Err>;

    #[get("xskscx.aspx?xh={stu_id}")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_default_exams(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<ExamsPage>, Self::Err>;

    #[post("xskscx.aspx?xh={stu_id}", "application/x-www-form-urlencoded; charset=gb2312")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_exams<'a>(
        &self,
        stu_id: &str,
        #[body] body: ExamsReq<'a>,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<ExamsPage>, Self::Err>;

    #[get("xscj.aspx?xh={stu_id}")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_scores_base(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<ScoresBasePage>, Self::Err>;

    #[post("xscj.aspx?xh={stu_id}", "application/x-www-form-urlencoded; charset=gb2312")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_scores<'a>(
        &self,
        stu_id: &str,
        #[body] body: ScoresReq<'a>,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<ScoresPage>, Self::Err>;

    #[get("xscj_zg.aspx?xh={stu_id}")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_major_scores(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<MajorScoresPage>, Self::Err>;

    #[get("xs_txsqddy.aspx?xh={stu_id}")]
    #[expect(200, "text/html; charset=gb2312")]
    async fn get_total_credit(
        &self,
        stu_id: &str,
        #[header(COOKIE)] cookie: String,
    ) -> Result<Response<TotalCreditPage>, Self::Err>;
}
