pub mod core;
use std::collections::HashMap;

use types::{config::ScrobConfig, integrations::Integrations};
pub mod integrations;
pub mod mb;
pub mod player;
use crate::integrations::base::BaseIntegrationTrait;

pub struct Context {
    pub integrations: HashMap<Integrations, Box<dyn BaseIntegrationTrait>>,
    pub config: ScrobConfig,
    pub preferences: Preferences,
}

#[derive(Debug)]
pub struct Preferences {
    pub disable_discord_rich_presence: bool,
    pub disable_lastfm_scrobble: bool,
}
