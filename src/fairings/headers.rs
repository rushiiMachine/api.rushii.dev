use rocket::response::Responder;
use rocket::{Request, Response};
use std::borrow::Cow;
use std::time::Duration;

/// Adds `Cache-Control` header with the value set to `public, max-age=<duration>`
#[derive(Debug, Clone, PartialEq)]
pub struct CacheControl<R>(pub Duration, pub R);

/// Adds the `Retry-After` header with the value set to the duration.
#[derive(Debug, Clone, PartialEq)]
pub struct RetryAfter<R>(pub Duration, pub R);

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for CacheControl<R> {
	fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'o> {
		let header = format!("public, max-age={}", self.0.as_secs());

		Response::build()
			.merge(self.1.respond_to(req)?)
			.raw_header("Cache-Control", Cow::from(header))
			.ok()
	}
}

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for RetryAfter<R> {
	fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'o> {
		let header = self.0.as_secs().to_string();

		Response::build()
			.merge(self.1.respond_to(req)?)
			.raw_header("Retry-After", Cow::from(header))
			.ok()
	}
}
