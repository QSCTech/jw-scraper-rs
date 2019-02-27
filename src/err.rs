#[derive(Fail, Debug)]
pub enum DeserializeError {
    #[fail(display = "{} mismatch {}", regex_ident, text)]
    RegexMismatch { regex_ident: String, text: String },

    #[fail(display = "reformation error: {}", err)]
    ReformationError { err: reformation::Error },

    #[fail(display = "other error: {:?}", err)]
    OtherError { err: Box<dyn failure::Fail> },
}

//impl<F: std::error::Error + Sync + Send + 'static> From<F> for DeserializeError {
//    fn from(fail: F) -> Self {
//        DeserializeError::OtherError {
//            err: Box::new(fail),
//        }
//    }
//}

impl From<reformation::Error> for DeserializeError {
    fn from(err: reformation::Error) -> Self {
        DeserializeError::ReformationError { err }
    }
}
