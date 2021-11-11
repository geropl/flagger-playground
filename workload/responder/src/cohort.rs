use anyhow::anyhow;
use rocket::{http::Status, request::{self, Request, FromRequest, Outcome}};

pub const HEADER_NAME: &'static str = "x-cohort";

#[derive(Debug)]
pub struct Cohort {
    pub id: String,
}
impl std::fmt::Display for Cohort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id.as_str())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Cohort {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one(HEADER_NAME) {
            Some(id) => Outcome::Success(Cohort { id: id.to_string() }),
            None => Outcome::Failure((Status::BadRequest, anyhow!("{} not set", HEADER_NAME)))
        }
    }
}