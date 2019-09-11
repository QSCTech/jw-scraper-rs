use interfacer_http::{http::HeaderValue, mime::Mime, MimeExt};
use std::str::FromStr;

/// compare content-type without parameters
///
/// ```rust
/// use zju_jw_scraper::helper::mime_matcher;
/// use interfacer_http::http::HeaderValue;
/// assert!(mime_matcher(
///     &"application/json; charset=utf-8".parse().unwrap(),
///     &HeaderValue::from_static("application/json; charset=gbk"),
/// ));
/// ```
pub fn mime_matcher(expect: &Mime, actual: &HeaderValue) -> bool {
    if let Ok(actual_str) = actual.to_str() {
        if let Ok(actual_content_type) = Mime::from_str(actual_str) {
            return expect.pure_type() == actual_content_type.pure_type();
        }
    }
    false
}
