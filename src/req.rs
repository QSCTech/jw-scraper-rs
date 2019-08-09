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
