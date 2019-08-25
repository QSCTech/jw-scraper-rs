#![feature(custom_attribute, param_attrs, async_closure)]
#![allow(unused_attributes)]

pub mod helper;

#[cfg(any(feature = "client", feature = "test"))]
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
