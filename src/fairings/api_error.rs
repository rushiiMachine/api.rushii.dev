use rocket::http::Status;
use rocket::response::{Responder, Result};
use rocket::serde::json::Json;
use rocket::{Request, Response};
use serde::Serialize;

/// Responder wrapper type for a value and ApiError
pub type ApiResult<T> = std::result::Result<T, ApiError>;

/// API responses for all possible API errors
#[derive(Debug)]
pub enum ApiError {
	/// Authentication is required on this route but is missing
	MissingAuthentication,

	/// The supplied authentication is invalid
	InvalidAuthentication,

	/// Requested route was not found
	InvalidRoute,

	/// Generic request parsing fail
	InvalidRequest,

	/// Generic internal server error usually if panic
	InternalServerError,

	/// Server is temporarily unavailable to process the request.
	TemporarilyUnavailable,

	/// Other unaccounted-for error
	Unknown(Status),
}

impl ApiError {
	/// Returns the (status code, error id, error description) for this [ApiError]
	fn data(&self) -> (Status, &'static str, &'static str) {
		match self {
			ApiError::MissingAuthentication => (
				Status::Unauthorized,
				"AUTH_REQUIRED",
				"Authentication is required on this endpoints!"),
			ApiError::InvalidAuthentication => (
				Status::Forbidden,
				"INVALID_AUTH",
				"Invalid authentication supplied"),
			ApiError::InvalidRoute => (
				Status::NotFound,
				"INVALID_ROUTE",
				"Route not found"),
			ApiError::InvalidRequest => (
				Status::BadRequest,
				"INVALID_REQUEST",
				"Invalid request"),
			ApiError::InternalServerError => (
				Status::InternalServerError,
				"INTERNAL_SERVER_ERROR",
				"The server encountered an internal error while processing this request"),
			ApiError::TemporarilyUnavailable => (
				Status::ServiceUnavailable,
				"TEMPORARILY_UNAVAILABLE",
				"The server is currently unable to process this request. Please try again later."),
			ApiError::Unknown(status) => (
				*status,
				"UNKNOWN",
				status.reason().unwrap_or("Unknown")),
		}
	}
}

impl<'r> Responder<'r, 'static> for ApiError {
	fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
		let (status, name, description) = self.data();
		let body = ApiErrorBody {
			error: true,
			code: name,
			description,
		};

		Response::build()
			.merge(Json(body).respond_to(request)?)
			.status(status)
			.ok()
	}
}

#[derive(Serialize)]
struct ApiErrorBody<'s> {
	error: bool,
	code: &'s str,
	description: &'s str,
}
