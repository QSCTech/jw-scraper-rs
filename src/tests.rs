use crate::raw::tests::Config;
use crate::{client::client, CourseInfo, CourseSemester::*, ExamSemester::*, JWService};

#[tokio::test]
async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let cookie = service.login(&config.stu_id, &config.password).await?;
    assert!(!cookie.is_empty());
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
    } = service.get_course_info("021E0040").await?;
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
async fn test_courses() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let cookie = service.login(&config.stu_id, &config.password).await?;
    let courses = service
        .get_courses(&config.stu_id, 2017.into(), FallAndWinter, &cookie)
        .await?;
    assert!(!courses.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_exams() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let cookie = service.login(&config.stu_id, &config.password).await?;
    let exams = service
        .get_exams(&config.stu_id, 2017.into(), Winter, &cookie)
        .await?;
    assert!(!exams.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_scores() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let cookie = service.login(&config.stu_id, &config.password).await?;
    let scores = service.get_scores(&config.stu_id, &cookie).await?;
    assert!(!scores.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_major_scores() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let cookie = service.login(&config.stu_id, &config.password).await?;
    let scores = service.get_major_scores(&config.stu_id, &cookie).await?;
    assert!(!scores.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_total_credit() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let service = client(config.jwb_base_url.parse()?);
    let cookie = service.login(&config.stu_id, &config.password).await?;
    let _ = service.get_total_credit(&config.stu_id, &cookie).await?;
    Ok(())
}
