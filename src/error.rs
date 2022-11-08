use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use jsonschema::output::{ErrorDescription, OutputUnit};

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    JsonSchema(VecDeque<OutputUnit<ErrorDescription>>),
    #[cfg(feature = "validator")]
    Validator(validator::ValidationErrors),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeJson(err) => <serde_json::Error as Display>::fmt(err, f),
            Self::JsonSchema(err) => err.fmt(f),
            #[cfg(feature = "validator")]
            Self::Validator(err) => <validator::ValidationErrors as Display>::fmt(err, f),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::SerdeJson(_) => StatusCode::BAD_REQUEST,
            Self::JsonSchema(_) => StatusCode::UNPROCESSABLE_ENTITY,
            #[cfg(feature = "validator")]
            Self::Validator(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::SerdeJson(err) => err.error_response(),
            Self::JsonSchema(err) => HttpResponse::build(self.status_code()).json(err),
            #[cfg(feature = "validator")]
            Self::Validator(err) => HttpResponse::build(self.status_code()).json(err),
        }
    }
}
