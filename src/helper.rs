use interfacer_http::{mime::Mime, http::HeaderValue, MimeExt};
use std::str::FromStr;

pub fn mime_matcher(expect: &Mime, actual: &HeaderValue) -> bool {
    if let Ok(actual_str) = actual.to_str() {
        if let Ok(actual_content_type) = Mime::from_str(actual_str) {
            return expect.pure_type() == actual_content_type.pure_type()
        }
    }
    false
}