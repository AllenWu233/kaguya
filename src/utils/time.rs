use chrono::offset::Local;

/// Get formattary time string
pub fn get_time_string() -> String {
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}
