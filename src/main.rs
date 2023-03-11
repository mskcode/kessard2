extern crate reqwest;

use core::fmt;
use std::time::{Duration, Instant};

use tokio;

// TODO add reqwest_middleware https://docs.rs/reqwest-middleware/latest/reqwest_middleware/

#[tokio::main]
async fn main() {
    let health_check_result = check_health("https://www.google.com").await;
    println!(
        "URL {} is {} ({}, {}), checked in {} ms",
        health_check_result.url,
        health_check_result.status,
        health_check_result.http_status_code,
        health_check_result
            .error_message
            .or_else(|| { Option::from(String::from("")) })
            .as_ref()
            .unwrap(),
        health_check_result.request_duration.as_millis()
    );
}

enum HealthCheckStatus {
    OK,
    ERROR,
}

impl fmt::Display for HealthCheckStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HealthCheckStatus::OK => write!(f, "OK"),
            HealthCheckStatus::ERROR => write!(f, "ERROR"),
        }
    }
}

struct HealthCheckResult {
    url: String,
    status: HealthCheckStatus,
    http_status_code: u16,
    request_duration: Duration,
    error_message: Option<String>,
}

async fn check_health(url: &str) -> HealthCheckResult {
    let start = Instant::now();
    let response_result = reqwest::get(url).await;
    let request_duration = start.elapsed();

    return match response_result {
        Ok(response) => {
            let status: HealthCheckStatus = if response.status().is_success() {
                HealthCheckStatus::OK
            } else {
                HealthCheckStatus::ERROR
            };
            HealthCheckResult {
                url: String::from(url),
                status: status,
                http_status_code: response.status().as_u16(),
                request_duration: request_duration,
                error_message: Option::None,
            }
        }
        Err(error) => HealthCheckResult {
            url: String::from(url),
            status: HealthCheckStatus::ERROR,
            http_status_code: 0,
            request_duration: request_duration,
            error_message: Option::from(error.to_string()),
        },
    };
}
