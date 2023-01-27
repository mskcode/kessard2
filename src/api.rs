use rocket::response::content::RawJson;
use rocket::{Build, Rocket};

pub fn initialize_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![get_healthz])
}

#[get("/healthz")]
fn get_healthz() -> RawJson<&'static str> {
    RawJson(r#"{"status":"OK"}"#)
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let rocket = super::initialize_rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::get_healthz)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), r#"{"status":"OK"}"#);
    }
}
