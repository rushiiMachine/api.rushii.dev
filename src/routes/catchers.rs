use rocket::catch;
use rocket::http::Status;
use rocket::Request;

use crate::fairings::api_error::ApiError;

#[catch(400)]
pub fn catch_400() -> ApiError {
	ApiError::InvalidRequest
}

#[catch(404)]
pub fn catch_404() -> ApiError {
	ApiError::InvalidRoute
}

#[catch(500)]
pub fn catch_500() -> ApiError {
	ApiError::InternalServerError
}

#[catch(default)]
pub fn catch_all(status: Status, _: &Request) -> ApiError {
	ApiError::Unknown(status)
}
