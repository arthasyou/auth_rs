pub mod msg;

use mongodb::bson;
// use serde::Serialize;
use thiserror::Error;


#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),

    #[error("to string error: {0}")]
    ToStrError(#[from] actix_web::http::header::ToStrError),
    #[error("actix_web error: {0}")]
    ActixWebError(#[from] actix_web::Error),
    #[error("auth error: {0}")]
    AuthError(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

// pub struct Error(Box<ErrorKind>);

// impl Error {
//     /// Return the specific type of this error.
//     pub fn get(&self) -> &ErrorKind {
//         &self.0
//     }

//     /// Unwrap this error into its underlying type.
//     pub fn into_kind(self) -> ErrorKind {
//         *self.0
//     }
// }

// pub fn new_error(kind: ErrorKind) -> Error {
//     Error(Box::new(kind))
// }


// #[derive(Serialize)]
// struct ErrorResponse {
//     message: String,
// }


