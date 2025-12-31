pub fn allowed_visibilities(user_tier: Option<&str>) -> Vec<&'static str> {
    match user_tier {
        None => vec!["PUBLIC"],
        Some("PUBLIC") => vec!["PUBLIC"],
        Some("REGISTERED") => vec!["PUBLIC"],
        Some("FRIEND") => vec!["PUBLIC", "FRIENDS"],
        Some("CLOSE_FRIEND") => vec!["PUBLIC", "FRIENDS", "CLOSE_FRIENDS"],
        Some("ADMIN") => vec!["PUBLIC", "FRIENDS", "CLOSE_FRIENDS", "PRIVATE", "CUSTOM"],
        _ => vec!["PUBLIC"],
    }
}

pub fn can_view_visibility(user_tier: Option<&str>, visibility: &str) -> bool {
    allowed_visibilities(user_tier).contains(&visibility)
}
