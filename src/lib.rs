#![feature(custom_attribute)]

use reformation::Reformation;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate unhtml_derive;

#[macro_use]
extern crate lazy_static;

use crate::err::DeserializeError;
use regex::Regex;
use std::str::FromStr;
use unhtml::{self, VecFromHtml};

#[derive(FromHtml)]
pub struct HiddenForm {
    #[html(selector = "input[type=\"hidden\"]:nth-child(1)", attr = "value")]
    pub event_target: String,

    #[html(selector = "input[type=\"hidden\"]:nth-child(2)", attr = "value")]
    pub event_argument: String,

    #[html(selector = "input[type=\"hidden\"]:nth-child(3)", attr = "value")]
    pub view_state: String,
}

#[derive(FromHtml)]
pub struct LoginPage {
    #[html(selector = "#Form1")]
    pub hidden_form: HiddenForm,
}

#[derive(FromHtml)]
pub struct SelectMenu {
    #[html(selector = "option[selected=\"selected\"]", attr = "value")]
    pub selected: String,

    #[html(selector = "option", attr = "value")]
    pub all_options: Vec<String>,
}

#[derive(FromHtml)]
pub struct Course {
    #[html(selector = "td:nth-child(1) > a", attr = "inner")]
    pub code: String,
    #[html(selector = "td:nth-child(2) > a", attr = "inner")]
    pub name: String,
    #[html(selector = "td:nth-child(3) > a", attr = "inner")]
    pub teacher_name: String,
    #[html(selector = "td:nth-child(4)", attr = "inner")]
    pub semester: String,
    #[html(selector = "td:nth-child(5)", attr = "inner")]
    pub times: String,
    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub places: String,
}

#[derive(FromHtml)]
pub struct CoursesPage {
    #[html(selector = "#xskb_form")]
    pub hidden_form: HiddenForm,

    #[html(selector = "#xnd")]
    pub school_year: SelectMenu,

    #[html(selector = "#xqd")]
    pub semester: SelectMenu,

    #[html(selector = "#xsgrid > tbody > tr:nth-child(1n + 2)")]
    pub courses: Vec<Course>,
}

/// match string like: 2019年01月13日(08:00-10:00)
pub struct ExamTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub start_hour: u32,
    pub start_min: u32,
    pub end_hour: u32,
    pub end_min: u32,
}

impl FromStr for ExamTime {
    type Err = DeserializeError;
    fn from_str(raw_time: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref EXAM_TIME_REGEX: Regex = Regex::new(
                r"(?x)
(?P<year>\d{4})  # the year
年
(?P<month>\d{2}) # the month
月
(?P<day>\d{2})   # the day
日\(
(?P<start_hour>\d{2})  # the start hour
:
(?P<start_min>\d{2})   # the start minute
-
(?P<end_hour>\d{2})    # the end hour
:
(?P<end_min>\d{2})     # the end minute
\)
"
            )
            .unwrap();
        }
        let time = EXAM_TIME_REGEX
            .captures(raw_time)
            .ok_or(DeserializeError::RegexMismatch {
                regex_ident: "EXAM_TIME_REGEX".to_owned(),
                text: raw_time.to_owned(),
            })?;
        Ok(Self {
            year: i32::from_str(&time["year"])?,
            month: u32::from_str(&time["month"])?,
            day: u32::from_str(&time["day"])?,
            start_hour: u32::from_str(&time["start_hour"])?,
            start_min: u32::from_str(&time["start_min"])?,
            end_hour: u32::from_str(&time["end_hour"])?,
            end_min: u32::from_str(&time["end_min"])?,
        })
    }
}

pub enum OptionalExamTime {
    Some(ExamTime),
    None,
}

impl FromStr for OptionalExamTime {
    type Err = DeserializeError;
    fn from_str(raw_time: &str) -> Result<Self, Self::Err> {
        match raw_time {
            "&nbsp;" => Ok(OptionalExamTime::None),
            other => Ok(OptionalExamTime::Some(ExamTime::from_str(other)?)),
        }
    }
}

#[derive(FromHtml)]
pub struct Exam {
    #[html(selector = "td:nth-child(1)", attr = "inner")]
    pub identifier: String,

    #[html(selector = "td:nth-child(2)", attr = "inner")]
    pub name: String,

    #[html(selector = "td:nth-child(3)", attr = "inner")]
    pub credit: f32,

    #[html(selector = "td:nth-child(4)", attr = "inner")]
    pub relearning: String,

    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub semester: String,

    #[html(selector = "td:nth-child(7)", attr = "inner")]
    pub final_exam_time: OptionalExamTime,

    #[html(selector = "td:nth-child(8)", attr = "inner")]
    pub final_exam_place: String,

    #[html(selector = "td:nth-child(9)", attr = "inner")]
    pub final_exam_seat: String,

    #[html(selector = "td:nth-child(10)", attr = "inner")]
    pub mid_exam_time: OptionalExamTime,

    #[html(selector = "td:nth-child(11)", attr = "inner")]
    pub mid_exam_place: String,

    #[html(selector = "td:nth-child(12)", attr = "inner")]
    pub mid_exam_seat: String,
}

#[derive(FromHtml)]
pub struct ExamsPage {
    #[html(selector = "#_ctl0")]
    pub hidden_form: HiddenForm,

    #[html(selector = "#xnd")]
    pub school_year: SelectMenu,

    #[html(selector = "#xqd")]
    pub semester: SelectMenu,

    #[html(selector = "#DataGrid1 > tbody > tr:nth-child(1n + 2)")]
    pub exams: Vec<Exam>,
}

#[derive(FromHtml)]
pub struct ObjectMovedPage {
    #[html(selector = "a", attr = "href")]
    pub to: String,
}

#[derive(FromHtml)]
pub struct MajorScoresPage {

}

#[derive(FromHtml)]
pub struct MajorScore {

}

pub struct MajorSummaryTable {

}

#[derive(Reformation)]
#[reformation(r"{key}={value}")]
pub struct KVPattern {
    pub key: String,
    pub value: f32
}

mod err;
#[cfg(test)]
mod tests;
