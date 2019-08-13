use interfacer_http::derive::ToContent;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct LoginBody<'a> {
    #[serde(rename = "__EVENTTARGET")]
    event_target: &'a str,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: &'a str,

    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "TextBox1")]
    username: &'a str,

    #[serde(rename = "TextBox2")]
    password: &'a str,

    #[serde(rename = "RadioButtonList1")]
    radio_button: &'a str,

    #[serde(rename = "Text1")]
    text1: &'a str,
}

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct CoursesReq<'a> {
    #[serde(rename = "__EVENTTARGET")]
    event_target: &'a str,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: &'a str,

    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "xxms")]
    repr_mode: &'a str,

    #[serde(rename = "xnd")]
    school_year: &'a str,

    #[serde(rename = "xqd")]
    semester: &'a str,

    #[serde(rename = "kcxx")]
    course_into: &'a str,
}

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct ExamsReq<'a> {
    #[serde(rename = "__EVENTTARGET")]
    event_target: &'a str,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: &'a str,

    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "xnd")]
    school_year: &'a str,

    #[serde(rename = "xqd")]
    semester: &'a str,
}

impl<'a> LoginBody<'a> {
    pub fn new(view_state: &'a str, username: &'a str, password: &'a str) -> Self {
        Self {
            event_target: "Button1",
            event_argument: "",
            view_state,
            username,
            password,
            radio_button: "学生",
            text1: "",
        }
    }
}

impl<'a> CoursesReq<'a> {
    pub fn new(
        event_target: &'a str,
        view_state: &'a str,
        school_year: &'a str,
        semester: &'a str,
    ) -> Self {
        Self {
            event_target,
            event_argument: "",
            view_state,
            repr_mode: "列表",
            school_year,
            semester,
            course_into: "",
        }
    }
}

impl<'a> ExamsReq<'a> {
    pub fn new(
        event_target: &'a str,
        view_state: &'a str,
        school_year: &'a str,
        semester: &'a str,
    ) -> Self {
        Self {
            event_target,
            event_argument: "",
            view_state,
            school_year,
            semester,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interfacer_http::{
        content_types::{APPLICATION_FORM, CHARSET_GB2312},
        ContentType, ToContent,
    };

    #[test]
    fn login_body() {
        let body = LoginBody::new(
            "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
            "3160100000",
            "123456",
        );
        let data: Vec<u8> = body
            .to_content(&ContentType::new(
                APPLICATION_FORM,
                Some(CHARSET_GB2312),
                None,
            ))
            .unwrap();

        assert_eq!(
            "__EVENTTARGET=Button1&__EVENTARGUMENT=&__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&TextBox1=3160100000&TextBox2=123456&RadioButtonList1=%D1%A7%C9%FA&Text1=",
            &String::from_utf8(data).unwrap()
        );
    }

    #[test]
    fn courses_req() {
        let req = CoursesReq::new(
            "xnd",
            "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
            "2018-2019",
            "1|秋、冬",
        );
        let data: Vec<u8> = req
            .to_content(&ContentType::new(
                APPLICATION_FORM,
                Some(CHARSET_GB2312),
                None,
            ))
            .unwrap();

        assert_eq!(
            "__EVENTTARGET=xnd&__EVENTARGUMENT=&__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&xxms=%C1%D0%B1%ED&xnd=2018-2019&xqd=1%7C%C7%EF%A1%A2%B6%AC&kcxx=",
            &String::from_utf8(data).unwrap()
        );
    }

    #[test]
    fn exams_req() {
        let req = ExamsReq::new(
            "xnd",
            "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
            "2018-2019",
            "2|春",
        );
        let data: Vec<u8> = req
            .to_content(&ContentType::new(
                APPLICATION_FORM,
                Some(CHARSET_GB2312),
                None,
            ))
            .unwrap();

        assert_eq!(
            "__EVENTTARGET=xnd&__EVENTARGUMENT=&__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&xnd=2018-2019&xqd=2%7C%B4%BA",
            &String::from_utf8(data).unwrap()
        );
    }
}
