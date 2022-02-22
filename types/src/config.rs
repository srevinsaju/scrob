use serde::{Deserialize, Serialize};

use crate::integrations::{Event, Integrations};

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordSettings {
    pub enabled: bool,
    pub blacklist_apps: Vec<String>,
    pub blacklist_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrobbleSettings {
    pub enabled: bool,
    pub blacklist_apps: Vec<String>,
    pub blacklist_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrobConfig {
    pub version: u8,
    pub password: String,
    pub username: String,
    pub discord: DiscordSettings,
    pub scrobble: ScrobbleSettings,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for ScrobConfig {
    fn default() -> Self {
        Self {
            version: 1,
            password: "".into(),
            username: "".into(),
            scrobble: ScrobbleSettings {
                enabled: true,
                blacklist_apps: vec![],
                blacklist_urls: vec![],
            },
            discord: DiscordSettings {
                enabled: true,
                blacklist_apps: vec![],
                blacklist_urls: vec![],
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScrobOperation {
    pub enabled: bool,
    pub event: Event,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScrobMessage {
    pub integration: Integrations,
    pub operation: ScrobOperation,
}
