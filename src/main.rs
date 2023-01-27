extern crate reqwest;

use core::fmt;

use tokio;

// TODO add reqwest_middleware https://docs.rs/reqwest-middleware/latest/reqwest_middleware/

#[tokio::main]
async fn main() {
    println!("Hello, Rust!");

    let health_check_result = check_health("https://www.google.com").await;
    println!("URL {} is {}", health_check_result.url, health_check_result.status);
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
}

async fn check_health(url: &str) -> HealthCheckResult {
    let response_result = reqwest::get(url).await;
    let status: HealthCheckStatus = if response_result.is_err() { HealthCheckStatus::ERROR } else { HealthCheckStatus::OK };
    return HealthCheckResult {
        url: String::from(url),
        status: status,
    };
}
