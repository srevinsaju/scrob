pub mod core;
use types::config::ScrobConfig;
mod integrations;
pub mod player;
use crate::integrations::base::BaseIntegrationTrait;

pub struct Context {
    pub integrations: Vec<Box<dyn BaseIntegrationTrait>>,
    pub config: ScrobConfig,
    pub preferences: Preferences,
}

#[derive(Debug)]
pub struct Preferences {
    pub enable_discord_rich_presence: bool,
    pub enable_lastfm_scrobble: bool,
}
