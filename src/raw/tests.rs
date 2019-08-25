use super::req::{
    CoursesReq, ExamsReq, LoginBody, ScoresReq, DEFAULT_COURSES_VIEW_STATE,
    DEFAULT_EXAMS_VIEW_STATE, LOGIN_VIEW_STATE, SCORES_BASE_VIEW_STATE,
};
use super::resp::{CourseInfo, HiddenForm};
use super::{RawJWService, JWB_COOKIE_NAME};
use crate::client::client;
use config::ConfigError;
use interfacer_http::{cookie::Cookie, ResponseExt};
use serde::{Deserialize, Serialize};

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
async fn test_login_page() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let login_page = RawJWService::get_login_page(&service).await?;
    assert_eq!(
        &login_page.body().hidden_form,
        &HiddenForm {
            event_argument: "".into(),
            event_target: "".into(),
            view_state: LOGIN_VIEW_STATE.into(),
        }
    );
    Ok(())
}

#[tokio::test]
async fn test_course_info() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let CourseInfo {
        code,
        name,
        college,
        credit,
        hours_per_week,
        prerequisite,
        intro,
    } = RawJWService::get_course_info(&service, "021E0040")
        .await?
        .into_body();
    assert_eq!("021E0040", &code);
    assert_eq!("马克思主义基本原理概论", &name);
    assert_eq!("马克思主义学院", &college);
    assert_eq!(2.5, credit);
    assert_eq!("2.0-1.0", &hours_per_week);
    assert_eq!("", &prerequisite);
    assert_eq!(
        "本课程旨在帮助学生从整体上把握马克思主义基本理论，从而把握人类社会发展的基本规律，以确立正确的世界观和人生观。教学内容主要有：物质世界及其发展规律；认识世界和改造世界的规律；人类社会及其发展规律；资本主义和社会主义发展规律；共产主义与人的自由全面发展规律。"
        , &intro
    );
    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    Ok(())
}

#[tokio::test]
async fn test_default_courses() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let courses = RawJWService::get_default_courses(&service, &config.stu_id, &cookie_str)
        .await?
        .into_body();
    assert_eq!(DEFAULT_COURSES_VIEW_STATE, &courses.hidden_form.view_state);
    assert!(courses.courses.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_courses() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let courses = RawJWService::get_courses(
        &service,
        &config.stu_id,
        CoursesReq::new(DEFAULT_COURSES_VIEW_STATE, "2018-2019", "1|秋、冬"),
        &cookie_str,
    )
    .await?
    .into_body();
    assert!(courses.courses.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_default_exams() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let exams = RawJWService::get_default_exams(&service, &config.stu_id, &cookie_str)
        .await?
        .into_body();
    assert_eq!(DEFAULT_EXAMS_VIEW_STATE, &exams.hidden_form.view_state);
    assert!(exams.exams.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_exams() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let exams = RawJWService::get_exams(
        &service,
        &config.stu_id,
        ExamsReq::new(DEFAULT_EXAMS_VIEW_STATE, "2018-2019", "1|冬"),
        &cookie_str,
    )
    .await?
    .into_body();
    assert!(exams.exams.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_scores_base() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let scores_base = RawJWService::get_scores_base(&service, &config.stu_id, &cookie_str)
        .await?
        .into_body();
    assert_eq!(SCORES_BASE_VIEW_STATE, &scores_base.hidden_form.view_state);
    Ok(())
}

#[tokio::test]
async fn test_scores() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let scores = RawJWService::get_scores(
        &service,
        &config.stu_id,
        ScoresReq::new(SCORES_BASE_VIEW_STATE),
        &cookie_str,
    )
    .await?
    .into_body();
    assert!(scores.scores.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_major_scores() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let major_scores = RawJWService::get_major_scores(&service, &config.stu_id, &cookie_str)
        .await?
        .into_body();
    assert!(major_scores.scores.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_total_credit() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let resp = RawJWService::login(
        &service,
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        ),
    )
    .await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let _ = RawJWService::get_total_credit(&service, &config.stu_id, &cookie_str)
        .await?
        .into_body();
    Ok(())
}
