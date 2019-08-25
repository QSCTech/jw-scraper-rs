use crate::raw::req::{
    CoursesReq, ExamsReq, LoginBody, ScoresReq, DEFAULT_COURSES_VIEW_STATE,
    DEFAULT_EXAMS_VIEW_STATE, LOGIN_VIEW_STATE, SCORES_BASE_VIEW_STATE,
};
use crate::raw::{RawJWService, JWB_COOKIE_NAME};
use crate::{
    Course, CourseInfo, CourseSemester, Exam, ExamSemester, MajorScore, SchoolYear, Score,
};
use interfacer_http::{async_trait, cookie::Cookie, HttpClient};

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
            <Self as RawJWService>::login(self, LoginBody::new(LOGIN_VIEW_STATE, stu_id, password))
                .await?;
        unimplemented!()
    }
    async fn get_course_info(&self, code: &str) -> Result<CourseInfo, Self::Err> {
        unimplemented!()
    }
    async fn get_course(
        &self,
        stu_id: &str,
        school_year: SchoolYear,
        semester: CourseSemester,
        cookie: &str,
    ) -> Result<Vec<Course>, Self::Err> {
        unimplemented!()
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
