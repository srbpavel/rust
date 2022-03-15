///ERRORS
use actix_web::{
    error,
    http::StatusCode,
    HttpResponse,
    Result,
};
use serde::Serialize;
use std::fmt;


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
                // this will be debug/error
                println!("Headers error occurred: {:?}", msg);
                "Headers error".into()
            },
            
            Self::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            },
            
            Self::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
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
