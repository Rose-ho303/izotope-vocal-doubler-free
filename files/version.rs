//! Static build info for Izotope Vocal Doubler Free.

pub const BUILD_VERSION: &str  = "v4.8.9";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "33c6bd05426a";
pub const BUILD_TIMESTAMP: u64 = 1730720617;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Izotope Vocal Doubler Free", BUILD_VERSION, BUILD_CHANNEL)
}
