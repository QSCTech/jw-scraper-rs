pub mod err;
use interfacer_http::derive::FromContent;
use reformation::Reformation;
use std::str::FromStr;
use unhtml::derive::{FromHtml, FromText};

#[derive(FromHtml)]
pub struct HiddenForm {
    #[html(
        selector = "input[type=\"hidden\"][name=\"__EVENTTARGET\"]",
        attr = "value"
    )]
    pub event_target: String,

    #[html(
        selector = "input[type=\"hidden\"][name=\"__EVENTARGUMENT\"]",
        attr = "value"
    )]
    pub event_argument: String,

    #[html(
        selector = "input[type=\"hidden\"][name=\"__VIEWSTATE\"]",
        attr = "value"
    )]
    pub view_state: String,
}

#[derive(FromHtml, FromContent)]
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

#[derive(FromHtml, FromContent)]
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
#[derive(Reformation, FromText)]
#[reformation(r"{year}年{month}月{day}日\({start_hour}:{start_min}-{end_hour}:{end_min}\)")]
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
    type Err = err::DeserializeError;
    fn from_str(raw_time: &str) -> Result<Self, Self::Err> {
        Ok(ExamTime::parse(raw_time)?)
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
    pub final_exam_time: Option<ExamTime>,

    #[html(selector = "td:nth-child(8)", attr = "inner")]
    pub final_exam_place: String,

    #[html(selector = "td:nth-child(9)", attr = "inner")]
    pub final_exam_seat: String,

    #[html(selector = "td:nth-child(10)", attr = "inner")]
    pub mid_exam_time: Option<ExamTime>,

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
    #[html(selector = "#DataGrid1 > tbody > tr:nth-child(1n + 2)")]
    pub scores: Vec<MajorScore>,

    #[html(selector = "#Label1", attr = "inner")]
    pub summary_table: MajorSummaryTable,
}

#[derive(FromHtml)]
pub struct MajorScore {
    #[html(selector = "td:nth-child(1)", attr = "inner")]
    pub identifier: String,

    #[html(selector = "td:nth-child(2)", attr = "inner")]
    pub course_name: String,

    #[html(selector = "td:nth-child(3)", attr = "inner")]
    pub raw_score: String,

    #[html(selector = "td:nth-child(4)", attr = "inner")]
    pub final_score: f32,

    #[html(selector = "td:nth-child(5)", attr = "inner")]
    pub credit: f32,

    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub grade_point: f32,

    #[html(selector = "td:nth-child(7)", attr = "inner")]
    pub school_year: String,
}

#[derive(Reformation, FromText)]
#[reformation(r"主修专业课程累计平均绩点={gpa}.*主修专业课程累计获得总学分={total_credit}")]
pub struct MajorSummaryTable {
    pub gpa: f32,
    pub total_credit: f32,
}

impl FromStr for MajorSummaryTable {
    type Err = err::DeserializeError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(data)?)
    }
}

#[derive(FromHtml)]
pub struct Score {
    #[html(selector = "td:nth-child(1)", attr = "inner")]
    pub identifier: String,

    #[html(selector = "td:nth-child(2)", attr = "inner")]
    pub course_name: String,

    #[html(selector = "td:nth-child(3)", attr = "inner")]
    pub raw_score: String,

    #[html(selector = "td:nth-child(4)", attr = "inner")]
    pub credit: f32,

    #[html(selector = "td:nth-child(5)", attr = "inner")]
    pub grade_point: f32,

    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub makeup_score: String,
}

#[derive(FromHtml)]
pub struct ScoresPage {
    #[html(selector = "#DataGrid1 > tbody > tr:nth-child(1n + 2)")]
    pub scores: Vec<Score>,
}

#[derive(FromHtml)]
pub struct ScoresBasePage {
    #[html(selector = "#Form1")]
    pub hidden_form: HiddenForm,

    #[html(selector = "#ddlXN")]
    pub school_year: SelectMenu,
}

#[derive(FromHtml)]
pub struct TotalCreditPage {
    #[html(selector = "#lb_yhxf", attr = "inner")]
    pub credit: f32,
}

#[derive(FromHtml)]
pub struct CourseInfo {
    #[html(selector = "#kcdm", attr = "inner")]
    pub code: String,
    #[html(selector = "#kczwmc", attr = "inner")]
    pub name: String,
    #[html(selector = "#kkxy", attr = "inner")]
    pub college: String,
    #[html(selector = "#xf", attr = "inner")]
    pub credit: f32,
    #[html(selector = "#zxs", attr = "inner")]
    pub hours_per_week: String,
}

#[cfg(test)]
mod tests;
