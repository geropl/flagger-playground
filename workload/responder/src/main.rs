use std::env;

#[macro_use] extern crate rocket;

mod cohort;
use cohort::*;

const VERSION_ENV_VAR: &str = "VERSION";

#[get("/")]
fn respond(cohort: Option<Cohort>) -> String {
    let version = env::var(VERSION_ENV_VAR)
        .unwrap_or("not-set".to_string());
    format!("{} ({:?})", version, cohort)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![respond])
}


#[cfg(test)]
mod test {
    use super::*;
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Header, Status};
    use defer_lite::defer;

    fn client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }

    #[test]
    fn respond_wo_version() {
        let client = client();
        let response = client
            .get("/")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn respond_wo_cohort_w_version() {
        let version = "abc";
        std::env::set_var(VERSION_ENV_VAR, version);
        defer! { std::env::remove_var(VERSION_ENV_VAR) };

        let client = client();
        let response = client
            .get("/")
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap().contains(version), true);
    }

    #[test]
    fn respond_w_cohort_w_version() {
        let version = "abc";
        let cohort_val = "ateam";
        std::env::set_var(VERSION_ENV_VAR, version);
        defer! { std::env::remove_var(VERSION_ENV_VAR) };

        let client = client();
        let response = client
            .get("/")
            .header(Header::new(cohort::HEADER_NAME, cohort_val))
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        let response_text = response.into_string().unwrap();
        assert_eq!(response_text.contains(version), true);
        assert_eq!(response_text.contains(cohort_val), true);
    }
}