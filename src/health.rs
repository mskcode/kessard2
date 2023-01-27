//use core::fmt;

// TODO add reqwest_middleware https://docs.rs/reqwest-middleware/latest/reqwest_middleware/

pub enum HealthCheckStatus {
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
}

pub async fn check_health(url: &str) -> HealthCheckResult {
    let response_result = reqwest::get(url).await;
    let status = match response_result.is_err() {
        false => HealthCheckStatus::OK,
        true => HealthCheckStatus::ERROR,
    };

    return HealthCheckResult {
        url: String::from(url),
        status: status,
    };
}
