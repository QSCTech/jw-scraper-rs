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
