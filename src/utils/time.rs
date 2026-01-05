use chrono::offset::{Local, Utc};

/// Get formattary time string
pub fn get_time_string() -> String {
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}

pub fn get_timestamp() -> String {
    Utc::now().timestamp().to_string()
}
