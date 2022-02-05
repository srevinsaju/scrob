pub mod core;
use std::collections::HashMap;

use types::{config::ScrobConfig, integrations::Integrations};
mod integrations;
pub mod player;
pub mod mb;
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
