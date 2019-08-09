use failure_derive::Fail;

#[derive(Fail, Debug)]
pub enum DeserializeError {
    #[fail(display = "reformation error: {}", err)]
    ReformationError { err: reformation::Error },
}

impl From<reformation::Error> for DeserializeError {
    fn from(err: reformation::Error) -> Self {
        DeserializeError::ReformationError { err }
    }
}
