#![feature(custom_attribute, param_attrs, async_closure)]

pub mod helper;

#[cfg(feature = "client")]
pub use client::{client, Client, Helper};

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

#[cfg(feature = "client")]
mod client;

#[cfg(all(test, feature = "client"))]
mod tests;
