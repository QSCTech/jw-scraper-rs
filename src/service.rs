use crate::raw::req::{
    CoursesReq, ExamsReq, LoginBody, ScoresReq, DEFAULT_COURSES_VIEW_STATE,
    DEFAULT_EXAMS_VIEW_STATE, LOGIN_VIEW_STATE, SCORES_BASE_VIEW_STATE,
};
use crate::raw::{RawJWService, JWB_COOKIE_NAME};
use crate::{
    Course, CourseInfo, CourseSemester, Exam, ExamSemester, MajorScore, SchoolYear, Score,
};
use interfacer_http::{
    async_trait,
    http::header::SET_COOKIE,
    http::{response::Parts, Response},
    HttpClient, ResponseExt, Unexpected,
};

#[async_trait]
pub trait JWService {
    type Err;
    async fn login(&self, stu_id: &str, password: &str) -> Result<String, Self::Err>;
    async fn get_course_info(&self, code: &str) -> Result<CourseInfo, Self::Err>;
    async fn get_course(
        &self,
        stu_id: &str,
        school_year: SchoolYear,
        semester: CourseSemester,
        cookie: &str,
    ) -> Result<Vec<Course>, Self::Err>;
    async fn get_exams(
        &self,
        stu_id: &str,
        school_year: SchoolYear,
        semester: ExamSemester,
        cookie: &str,
    ) -> Result<Vec<Exam>, Self::Err>;
    async fn get_scores(&self, stu_id: &str, cookie: &str) -> Result<Vec<Score>, Self::Err>;
    async fn get_major_scores(
        &self,
        stu_id: &str,
        cookie: &str,
    ) -> Result<Vec<MajorScore>, Self::Err>;
    async fn get_total_credit(&self, stu_id: &str, cookie: &str) -> Result<f32, Self::Err>;
}

#[async_trait]
impl<T> JWService for T
where
    T: HttpClient,
{
    type Err = <Self as HttpClient>::Err;

    async fn login(&self, stu_id: &str, password: &str) -> Result<String, Self::Err> {
        let resp =
            RawJWService::login(self, LoginBody::new(LOGIN_VIEW_STATE, stu_id, password)).await?;
        let cookies = resp.cookie_map().map_err(|err| {
            let mut copied_resp = Response::new(Vec::new());
            *copied_resp.status_mut() = resp.status();
            *copied_resp.version_mut() = resp.version();
            *copied_resp.headers_mut() = resp.headers().clone();
            Unexpected::new((SET_COOKIE, err.to_string()).into(), copied_resp)
        })?;
        let result = cookies.get(JWB_COOKIE_NAME);
        match result {
            Some(cookies) if cookies.len() > 0 => Ok(cookies[0].to_string()),
            _ => Err(Unexpected::new(
                (
                    SET_COOKIE,
                    format!("cookie named '{}' not found", JWB_COOKIE_NAME),
                )
                    .into(),
                Response::from_parts(resp.into_parts().0, Vec::new()),
            )
            .into()),
        }
    }
    async fn get_course_info(&self, code: &str) -> Result<CourseInfo, Self::Err> {
        Ok(RawJWService::get_course_info(self, code).await?.into_body())
    }
    async fn get_course(
        &self,
        stu_id: &str,
        school_year: SchoolYear,
        semester: CourseSemester,
        cookie: &str,
    ) -> Result<Vec<Course>, Self::Err> {
        let school_year_str = school_year.to_string();
        Ok(RawJWService::get_courses(
            self,
            stu_id,
            CoursesReq::new(DEFAULT_COURSES_VIEW_STATE, &school_year_str, &semester),
            cookie,
        )
        .await?
        .into_body()
        .courses)
    }
    async fn get_exams(
        &self,
        stu_id: &str,
        school_year: SchoolYear,
        semester: ExamSemester,
        cookie: &str,
    ) -> Result<Vec<Exam>, Self::Err> {
        unimplemented!()
    }
    async fn get_scores(&self, stu_id: &str, cookie: &str) -> Result<Vec<Score>, Self::Err> {
        unimplemented!()
    }
    async fn get_major_scores(
        &self,
        stu_id: &str,
        cookie: &str,
    ) -> Result<Vec<MajorScore>, Self::Err> {
        unimplemented!()
    }
    async fn get_total_credit(&self, stu_id: &str, cookie: &str) -> Result<f32, Self::Err> {
        unimplemented!()
    }
}
