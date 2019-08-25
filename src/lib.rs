#![feature(custom_attribute, param_attrs, async_closure)]
#![allow(unused_attributes)]

pub mod helper;

#[cfg(any(feature = "client", feature = "test"))]
pub mod client;

#[doc(inline)]
pub use raw::resp::{Course, CourseIdentifier, CourseInfo, Exam, ExamTime, Score, MajorScore};
mod raw;
