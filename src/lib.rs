//! ### ZJU-JWB scraper
//!
//! [![Build status](https://img.shields.io/travis/QSCTech/jw-scraper-rs/master.svg)](https://travis-ci.org/QSCTech/jw-scraper-rs)
//! [![Coverage Status](https://coveralls.io/repos/github/QSCTech/jw-scraper-rs/badge.svg?branch=master)](https://coveralls.io/github/QSCTech/jw-scraper-rs?branch=master)
//! [![Crate version](https://img.shields.io/crates/v/zju-jw-scraper.svg)](https://crates.io/crates/zju-jw-scraper)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/QSCTech/jw-scraper-rs/blob/master/LICENSE)
//! [![Rust Docs](https://docs.rs/zju-jw-scraper/badge.svg)](https://docs.rs/zju-jw-scraper)
//!
//! #### service
//!
//! ```rust,ignore
//! #[async_trait]
//! pub trait JWService {
//!     type Err;
//!     async fn login(&self, stu_id: &str, password: &str) -> Result<String, Self::Err>;
//!     async fn get_course_info(&self, code: &str) -> Result<CourseInfo, Self::Err>;
//!     async fn get_courses(
//!         &self,
//!         stu_id: &str,
//!         school_year: SchoolYear,
//!         semester: CourseSemester,
//!         cookie: &str,
//!     ) -> Result<Vec<Course>, Self::Err>;
//!     async fn get_exams(
//!         &self,
//!         stu_id: &str,
//!         school_year: SchoolYear,
//!         semester: ExamSemester,
//!         cookie: &str,
//!     ) -> Result<Vec<Exam>, Self::Err>;
//!     async fn get_scores(&self, stu_id: &str, cookie: &str) -> Result<Vec<Score>, Self::Err>;
//!     async fn get_major_scores(
//!         &self,
//!         stu_id: &str,
//!         cookie: &str,
//!     ) -> Result<Vec<MajorScore>, Self::Err>;
//!     async fn get_total_credit(&self, stu_id: &str, cookie: &str) -> Result<f32, Self::Err>;
//! }
//! ```
//!
//! The `JWService` is implemented for all types which implements [`interfacer_http::HttpClient`](https://docs.rs/interfacer-http/0.2/interfacer_http/trait.HttpClient.html).
//!
//! ```rust,ignore
//! #[async_trait]
//! pub trait HttpClient: Sync {
//!     type Err: Error;
//!     async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err>;
//!     fn helper(&self) -> &Helper;
//! }
//! ```
//!
//! You can refer to `src/test.rs` for all use cases.
//!
//! #### client
//!
//! You can use this crate with `client` feature, like this:
//!
//! ```toml
//! zju-jw-scraper = { version = "0.2", features = ["client"] }
//! ```
//!
//! Then, you can use default client provided by `interfacer-http-hyper`:
//!
//! ```rust,no_run
//! use zju_jw_scraper::{client::client, JWService};
//!
//! #[tokio::test]
//! async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
//!     let service = client("http://jwbinfosys.zju.edu.cn".parse()?);
//!     let cookie = service.login("319000000", "test").await?;
//!     assert!(!cookie.is_empty());
//!     Ok(())
//! }
//! ```
//!
//! However, the default client can only handle requests on HTTP, if you want to use other protocol like HTTPS, you need other [`Connect`](https://docs.rs/hyper/0.13.0-alpha.1/hyper/client/connect/trait.Connect.html).
//!
//! As the example `connector`:
//!
//! ```rust,no_run
//! use hyper_tls::HttpsConnector;
//! use zju_jw_scraper::{client::client_on, JWService};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let service = client_on("https://jw.zjuqsc.com".parse()?, HttpsConnector::new()?);
//!     let cookie = service.login("319000000", "test").await?;
//!     assert!(!cookie.is_empty());
//!     Ok(())
//! }
//!
//! ```
//!
//! #### test
//!
//! Run basic tests:
//!
//! ```bash
//! cargo test
//! ```
//!
//! To test this crate fully, create a config file `test-settings.toml`, and fill it referring to `test-settings.toml.sample`. Then run tests:
//!
//! ```bash
//! cargo test --all --all-features
//! ```
//!
//! As an alternative, you can configure tests using environment variables with prefix `TEST_`.
//!
//! ```bash
//! TEST_stu_id=3190000000 cargo test --all --all-features
//! ```
//!
//! > Environment variables always have higher priority.
//!

#![feature(custom_attribute, param_attrs, async_closure)]

pub mod helper;

#[cfg(feature = "client")]
pub mod client;

#[doc(inline)]
pub use raw::resp::{
    Course, CourseIdentifier, CourseInfo, Exam, ExamTime, MajorScore, ObjectMovedPage, Score,
};

#[doc(inline)]
pub use req::{CourseSemester, ExamSemester, SchoolYear};

#[doc(inline)]
pub use service::JWService;

mod raw;
mod req;
mod service;

#[cfg(all(test, feature = "client"))]
mod tests;
