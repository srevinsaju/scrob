use serde::{Deserialize, Serialize};

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
