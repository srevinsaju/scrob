use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;





#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub track: String,
    pub artist: String,
    pub album: String,
    pub is_playing: bool,
    pub source: String,
    pub url: String,
    pub scrobble: bool,
    pub duration: Duration,
    pub album_art: String,
    pub mbid: String,
    pub artist_mbid: String,
    pub position: Duration,
    pub is_repeat: bool,
}

impl Song {
    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn from_json(data: &str) -> Song {
        serde_json::from_str(data).unwrap()
    }
    pub fn new() -> Song {
        Song {
            artist: "".to_string(),
            album_art: "".to_string(),
            album: "".to_string(),
            is_playing: false,
            source: "".to_string(),
            is_repeat: false,
            track: "".to_string(),
            duration: Duration::new(0, 0),
            position: Duration::new(0, 0),
            scrobble: false,
            url: "".to_string(),
            mbid: "".to_string(),
            artist_mbid: "".to_string(),
        }
    }
}
