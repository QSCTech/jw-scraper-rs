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
    radio_button: &'a [u8],

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
            radio_button: &[0xD1, 0xA7, 0xC9, 0xFA], // encoding in gbk: "学生"
            text1: "",
        }
    }
}

#[cfg(all(test, feature = "test"))]
mod tests {
    use super::*;
    use interfacer_http::{ToContent, ContentType, content_types::APPLICATION_FORM};

    #[test]
    fn login_body() {
        let body = LoginBody::new("qtqy470yt70q", "3160100000", "123456");
        let data: Vec<u8> = body.to_content(&ContentType::new(APPLICATION_FORM, None, None)).unwrap();
        println!("{}", &String::from_utf8_lossy(data.as_slice()));
    }
}
