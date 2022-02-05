use config::{DISCORD_APPID_ELISA, DISCORD_APPID_SPOTIFY, DISCORD_APPID_YOUTUBE_MUSIC, DISCORD_APPID_YOUTUBE, DISCORD_APPID_LOLLYPOP, DISCORD_APPID_GENERIC, DISCORD_APPID_GROOVE_MUSIC};
use serde::{Serialize, Deserialize};


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Integrations {
    Discord,
    Lastfm,
    Lyrix,
    Notification,
}


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Event {
    SongChanged,
    SongPaused,
    SongPlaying,
    NoMediaPlayer,
}



#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Players {
    Spotify,
    YoutubeMusic,
    Youtube,
    Lollypop,
    Vlc,
    Elisa,
    GenericMusicPlayer,
    Rhythmbox,
    GrooveMusic,
}

impl Players {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Players::Spotify => "spotify",
            Players::Elisa => "elisa",
            Players::YoutubeMusic => "youtube-music",
            Players::Youtube => "youtube",
            Players::Lollypop => "lollypop",
            Players::Vlc => "vlc",
            Players::GenericMusicPlayer => "music",
            Players::Rhythmbox => "rhythmbox",
            Players::GrooveMusic => "groove-music"
        }
    }

    pub fn as_display_str(&self) -> &'static str {
        match *self {
            Players::Spotify => "Spotify",
            Players::Elisa => "Elisa",
            Players::YoutubeMusic => "Youtube Music",
            Players::Youtube => "Youtube",
            Players::Lollypop => "Lollypop",
            Players::Vlc => "VLC",
            Players::GenericMusicPlayer => "Music",
            Players::Rhythmbox => "Rhythmbox",
            Players::GrooveMusic => "Groove Music"
        }
    }

    pub fn as_discord_app_id(&self) -> &'static str {
        match *self {
            Players::Spotify => DISCORD_APPID_SPOTIFY,
            Players::YoutubeMusic => DISCORD_APPID_YOUTUBE_MUSIC,
            Players::Youtube => DISCORD_APPID_GENERIC,
            Players::Lollypop => DISCORD_APPID_LOLLYPOP,
            Players::Vlc => DISCORD_APPID_GENERIC,
            Players::Elisa => DISCORD_APPID_ELISA,
            Players::GenericMusicPlayer => DISCORD_APPID_GENERIC,
            Players::Rhythmbox => DISCORD_APPID_GENERIC,
            Players::GrooveMusic => DISCORD_APPID_GROOVE_MUSIC,
        }
    }
}