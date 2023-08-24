use actix_session::SessionInsertError;
use serde_json;
use serde::Serialize;

use actix_web::{http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Display, Debug)]
#[display(fmt = "status: {}", status)]
pub struct HttpErrorResponse {
    status: StatusCode,
    message: String,
}

impl HttpErrorResponse {
    pub fn get_status(&self) -> StatusCode {
        self.status
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl From<anyhow::Error> for HttpErrorResponse {
    fn from(e: anyhow::Error) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<SessionInsertError> for HttpErrorResponse {
    fn from(e: SessionInsertError) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<mongodb::error::Error> for HttpErrorResponse {
    fn from(e: mongodb::error::Error) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<bson::oid::Error> for HttpErrorResponse {
    fn from(e: bson::oid::Error) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for HttpErrorResponse {
    fn from(e: validator::ValidationErrors) -> Self {
        let x = e.errors();
        if x.contains_key("email") {
            return HttpErrorResponse {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_email".to_string(),
            };
        }
        if x.contains_key("password") {
            return HttpErrorResponse {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_password".to_string(),
            };
        }
        if x.contains_key("fname") {
            return HttpErrorResponse {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_fname".to_string(),
            };
        }
        if x.contains_key("lname") {
            return HttpErrorResponse {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_lname".to_string(),
            };
        }
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

//
impl From<String> for HttpErrorResponse {
    fn from(e: String) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e,
        }
    }
}

impl From<&str> for HttpErrorResponse {
    fn from(e: &str) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<actix_web::Error> for HttpErrorResponse {
    fn from(e: actix_web::Error) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for HttpErrorResponse {
    fn from(e: serde_json::Error) -> Self {
        HttpErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    message: String,
}

impl actix_web::ResponseError for HttpErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.get_status()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            status: self.status_code().as_u16(),
            message: self.get_message()
        })
    }
}