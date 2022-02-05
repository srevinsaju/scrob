use serde::{Deserialize, Serialize};
use serde_json;
use std::time::{Duration, SystemTime};

use crate::integrations::Players;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub track: String,
    pub artist: String,
    pub album: String,
    pub is_playing: bool,
    pub source: Players,
    pub url: String,
    pub scrobble: bool,
    pub duration: Duration,
    pub album_art: String,
    pub mbid: String,
    pub artist_mbid: String,
    pub position: Duration,
    pub is_repeat: bool,
    pub start_time: SystemTime,
}

impl Song {
    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn from_json(data: &str) -> Song {
        serde_json::from_str(data).unwrap()
    }

    /// creates an empty song instance with defaults for all fields
    pub fn new() -> Song {
        Song {
            artist: "".to_string(),
            album_art: "".to_string(),
            album: "".to_string(),
            is_playing: false,
            source: Players::GenericMusicPlayer,
            is_repeat: false,
            track: "".to_string(),
            duration: Duration::new(0, 0),
            position: Duration::new(0, 0),
            start_time: SystemTime::now(),
            scrobble: false,
            url: "".to_string(),
            mbid: "".to_string(),
            artist_mbid: "".to_string(),
        }
    }

    /// gets the first artist, by splitting on "," and then on "&"
    pub fn get_first_artist(&self) -> String {
        let mut artist: String = "".to_string();
        if let Some(hmm) = self.artist.split(",").next() {
            if let Some(hmm) = hmm.split("&").next() {
                artist = hmm.to_string();
            }
        }
        artist.to_string()
    }
}
