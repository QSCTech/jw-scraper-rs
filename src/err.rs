#[derive(Fail, Debug)]
pub enum DeserializeError {
    #[fail(display = "{} mismatch {}", regex_ident, text)]
    RegexMismatch { regex_ident: String, text: String },

    #[fail(display = "other error: {:?}", err)]
    OtherError { err: Box<dyn failure::Fail> },
}

impl<F: std::error::Error + Sync + Send + 'static> From<F> for DeserializeError {
    fn from(fail: F) -> Self {
        DeserializeError::OtherError {
            err: Box::new(fail),
        }
    }
}
