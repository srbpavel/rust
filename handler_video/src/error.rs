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
    status: String,
}

impl VideoError {
    fn error_response(&self) -> String {
        match self {
            Self::HeadersError(msg) => {
                // this goes to server log
                // ERROR handler_video::error]
                error!("Headers error occurred: {:?}", msg);

                // this goes to user as response
                // + also to server log
                // DEBUG actix_web::middleware::logger]
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
    // here we define which status code will error have
    fn status_code(&self) -> StatusCode {
        match self {
            Self::HeadersError(_msg) | Self::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            
            Self::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    // this is the json response we get after invalid input
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(
                // our err resp struct
                MyErrorResponse {
                    error_message: self.error_response(),
                    status: format!("{} / {} / {}",
                                    self.status_code().as_str(),
                                    self.status_code().as_u16(),
                                    // Display Trait
                                    self.status_code(),
                                    )
                }
            )
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
