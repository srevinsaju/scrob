use serde::{Deserialize, Serialize};

use crate::integrations::{Event, Integrations, Players};

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OperationType {
    Event,
    CustomPlayer,
    Integration,
    Null,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScrobOperation {
    pub type_: OperationType,

    pub enabled: bool,

    pub event: Event,
    pub integration: Integrations,
    pub custom_player: Players,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScrobMessage {
    pub operation: ScrobOperation,
}

impl ::std::default::Default for ScrobMessage {
    fn default() -> ScrobMessage {
        ScrobMessage {
            operation: ScrobOperation::default(),
        }
    }
}

impl ::std::default::Default for ScrobOperation {
    fn default() -> ScrobOperation {
        ScrobOperation {
            type_: OperationType::Null,
            enabled: false,
            event: Event::Null,
            integration: Integrations::Null,
            custom_player: Players::Null,
        }
    }
}
