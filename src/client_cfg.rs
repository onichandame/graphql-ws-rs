use std::time::Duration;

#[derive(Default, Clone)]
pub struct ClientConfig {
    pub url: String,
    /// disables ping if 0
    pub ping_interval: Duration,
}
