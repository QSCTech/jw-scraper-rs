use crate::raw::{RawJWService, JWB_COOKIE_NAME};
use crate::raw::resp::{LoginPage, HiddenForm, CoursesPage, ExamsPage, ScoresBasePage, ScoresPage, MajorScoresPage, TotalCreditPage};
use crate::raw::req::{LoginBody, CoursesReq, ExamsReq, ScoresReq, LOGIN_VIEW_STATE, DEFAULT_COURSES_VIEW_STATE, DEFAULT_EXAMS_VIEW_STATE, SCORES_BASE_VIEW_STATE};
use config::ConfigError;
use interfacer_http::{Helper, http::Response, ResponseExt, cookie::Cookie};
use interfacer_http_hyper::Client;
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
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let login_page: Response<LoginPage> = service.get_login_page().await?;
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
async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    Ok(())
}

#[tokio::test]
async fn test_default_courses() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let courses: CoursesPage = service.get_default_courses(&config.stu_id, cookie_str).await?.into_body();
    assert_eq!(DEFAULT_COURSES_VIEW_STATE, &courses.hidden_form.view_state);
    assert!(courses.courses.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_courses() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let courses: CoursesPage = service.get_courses(
        &config.stu_id,
        CoursesReq::new(DEFAULT_COURSES_VIEW_STATE, "2018-2019", "1|秋、冬"),
        cookie_str,
    ).await?.into_body();
    assert!(courses.courses.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_default_exams() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let exams: ExamsPage = service.get_default_exams(&config.stu_id, cookie_str).await?.into_body();
    assert_eq!(DEFAULT_EXAMS_VIEW_STATE, &exams.hidden_form.view_state);
    assert!(exams.exams.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_exams() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let exams: ExamsPage = service.get_exams(
        &config.stu_id,
        ExamsReq::new(DEFAULT_EXAMS_VIEW_STATE, "2018-2019", "1|冬"),
        cookie_str,
    ).await?.into_body();
    assert!(exams.exams.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_scores_base() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let scores_base: ScoresBasePage = service.get_scores_base(&config.stu_id, cookie_str).await?.into_body();
    assert_eq!(SCORES_BASE_VIEW_STATE, &scores_base.hidden_form.view_state);
    Ok(())
}

#[tokio::test]
async fn test_scores() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let scores: ScoresPage = service.get_scores(
        &config.stu_id,
        ScoresReq::new(SCORES_BASE_VIEW_STATE),
        cookie_str,
    ).await?.into_body();
    assert!(scores.scores.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_major_scores() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let major_scores: MajorScoresPage = service.get_major_scores(&config.stu_id, cookie_str).await?.into_body();
    assert!(major_scores.scores.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_total_credit() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = Client::new().with_helper(
        Helper::new()
            .with_base_url(config.jwb_base_url.parse()?)
    );
    let resp: Response<()> = service.login(
        LoginBody::new(
            LOGIN_VIEW_STATE,
            config.stu_id.as_str(),
            config.password.as_str(),
        )
    ).await?;
    let cookies = resp.cookie_map()?;
    let cookie = cookies.get(JWB_COOKIE_NAME);
    assert!(cookie.is_some());
    assert_eq!(1, cookie.unwrap().len());
    let (name, value) = cookie.unwrap()[0].name_value();
    let cookie_str = Cookie::new(name.to_owned(), value.to_owned()).to_string();
    let _: TotalCreditPage = service.get_total_credit(&config.stu_id, cookie_str).await?.into_body();
    Ok(())
}
