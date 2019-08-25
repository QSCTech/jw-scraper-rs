#![feature(custom_attribute, param_attrs, async_closure)]
#![allow(unused_attributes)]

pub mod helper;

#[cfg(any(feature = "client", feature = "test"))]
pub mod client;

#[doc(inline)]
pub use raw::resp::{Course, CourseIdentifier, CourseInfo, Exam, ExamTime, MajorScore, Score};

#[doc(inline)]
pub use req::{CourseSemester, ExamSemester, SchoolYear};

mod raw;
mod req;
mod service;
