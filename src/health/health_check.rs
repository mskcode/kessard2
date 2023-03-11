extern crate reqwest;

use std::fmt;
use std::time::{Duration, Instant};

// TODO add reqwest_middleware https://docs.rs/reqwest-middleware/latest/reqwest_middleware/

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

pub struct HealthCheckResult {
    url: String,
    status: HealthCheckStatus,
    http_status_code: u16,
    request_duration: Duration,
    error_message: Option<String>,
}

impl HealthCheckResult {
    pub fn dump(&self) {
        let error_message_or_blank = self
            .error_message
            .as_ref()
            .map(String::as_str)
            .unwrap_or("");

        println!(
            "URL {} is {} ({}, {}), checked in {} ms",
            self.url,
            self.status,
            self.http_status_code,
            error_message_or_blank,
            self.request_duration.as_millis()
        );
    }
}

pub async fn check_health(url: &str) -> HealthCheckResult {
    let start = Instant::now();
    // we're intentionally not using HTTP connection pooling since we want to
    // establish a new connection every time
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
