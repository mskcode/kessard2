use crate::health::health_check::check_health;
use actix_web::{get, http::header::ContentType, web, HttpResponse};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_healthz).service(get_workload_health);
}

#[get("/healthz")]
async fn get_healthz() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(r#"{"status":"OK"}"#)
}

#[get("/h/{name}")]
async fn get_workload_health(name: web::Path<String>) -> HttpResponse {
    println!("{}", name);

    let result = check_health("https://www.google.com").await;
    result.dump();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(r#"{"status":"OK"}"#)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{
//         http::{self, header::ContentType},
//         test,
//     };

//     #[actix_web::test]
//     async fn test_get_healthz_ok() {
//         let req = test::TestRequest::default().to_http_request();
//         let resp = get_healthz(req).await;
//         assert_eq!(resp.status(), http::StatusCode::OK);
//     }
// }
