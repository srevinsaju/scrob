use serde::{Deserialize, Serialize};

use crate::integrations::{Event, Integrations};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrobConfig {
    pub version: u8,
    pub password: String,
    pub username: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for ScrobConfig {
    fn default() -> Self {
        Self {
            version: 1,
            password: "".into(),
            username: "".into(),
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
