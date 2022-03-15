///ERRORS
use actix_web::{
    error,
    http::StatusCode,
    HttpResponse,
    Result,
};
use serde::Serialize;
use std::fmt;

use log::{debug,
          error,
};

#[derive(Debug, Serialize)]
pub enum VideoError {
    HeadersError(String),
    ActixError(String),
    NotFound(String),
}


#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl VideoError {
    fn error_response(&self) -> String {
        match self {
            Self::HeadersError(msg) => {
                // this goes to server log
                error!("Headers error occurred: {:?}", msg);
                // this goes to user as response
                // + also as DEBUG to server runtime
                // via -> actix_web::middleware::logger
                //"Headers error".into()
                format!("{}",
                        msg,
                )
                    .into()
            },
            
            Self::ActixError(msg) => {
                error!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            },
            
            Self::NotFound(msg) => {
                error!("Not found error occurred: {:?}", msg);
                msg.into()
            },
        }
    }
}

impl error::ResponseError for VideoError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::HeadersError(_msg) | Self::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            
            Self::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(MyErrorResponse {
                error_message: self.error_response(),
            })
    }
}

impl fmt::Display for VideoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for VideoError {
    fn from(err: actix_web::error::Error) -> Self {
        Self::ActixError(err.to_string())
    }
}

/*
impl From<HeadersError> for VideoError {
    fn from(err: HeadersError) -> Self {
        Self::HeadersError(err.to_string())
    }
}
 */
