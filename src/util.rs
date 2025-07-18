use chrono::Utc;

pub fn today_gmt() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

pub fn rate_limit(_guild: &str, _user: &str) -> bool {
    // Simplified rate limiting - always allow for now
    // In a real implementation, we'd use fastly::cache
    false
}
