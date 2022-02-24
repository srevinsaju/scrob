use serde::{Deserialize, Serialize};

use crate::integrations::{Event, Integrations};

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordSettings {
    pub enabled: bool,
    pub blacklist_apps: Vec<String>,
    pub blacklist_urls: Vec<String>,
    pub blacklist_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LastfmSettings {
    pub enabled: bool,
    pub username: String,
    pub password: String,
    pub blacklist_apps: Vec<String>,
    pub blacklist_urls: Vec<String>,
    pub blacklist_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrobConfig {
    pub version: u8,
    pub discord: DiscordSettings,
    pub lastfm: LastfmSettings,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for ScrobConfig {
    fn default() -> Self {
        Self {
            version: 1,
            lastfm: LastfmSettings {
                enabled: true,
                username: "".to_string(),
                password: "".to_string(),
                blacklist_apps: vec![],
                blacklist_urls: vec![],
                blacklist_ids: vec![],
            },
            discord: DiscordSettings {
                enabled: true,

                // by default, ignore spotify, because spotify has its own discord
                // encoder
                blacklist_apps: vec!["spotify".to_string()],
                blacklist_urls: vec![],
                blacklist_ids: vec![],
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
