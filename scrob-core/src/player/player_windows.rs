use types::{integrations::Players, song::Song};

use log::trace;
use log::warn;
use std::time::Duration;
use std::time::SystemTime;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager as MediaManager;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus as MediaStatus;
use windows::Media::MediaTimelineController;

/// Uses the winrt api to get access the media player data.
pub fn get_current_song() -> Result<Song, &'static str> {
    // TODO: figure out how to get the position of the song
    let manager = MediaManager::RequestAsync().expect("Failed to connect to Windows Media Manager");
    let session = manager
        .get()
        .expect("Failed to get sessions from Windows Media Manager");
    let current_session = session.GetCurrentSession();
    if let Err(e) = current_session {
        warn!("No active sessions detected, skipping: {}", e);
        return Ok(Song::new());
    }

    let current_session = current_session.expect("Failed to instatiate current session");
    let info = current_session.TryGetMediaPropertiesAsync();
    if let Err(e) = info {
        warn!("No song metadata was received: {}", e);
        return Ok(Song::new());
    }
    let info = info.unwrap();

    let mut is_playing = true;

    let current_session_playback_info = current_session.GetPlaybackInfo();
    if let Ok(playback_info) = current_session_playback_info {
        is_playing = playback_info
            .PlaybackStatus()
            .unwrap_or(MediaStatus::Playing)
            == MediaStatus::Playing;
    }

    let mut duration = Duration::new(0, 0);
    let mut position = Duration::new(0, 0);

    let current_session_timeline_info = current_session.GetTimelineProperties();
    if let Ok(timeline_info) = current_session_timeline_info {
        let end_time = timeline_info
            .EndTime()
            .expect("Failed to get end time of media");
        duration = end_time.into();

        let current_time = timeline_info.Position().expect("Failed to get position");
        position = current_time.into();
    }

    let mut source = Players::GenericMusicPlayer;
    if let Ok(origin) = current_session.SourceAppUserModelId() {
        let origin = origin.to_string();
        trace!("Detected song from '{}'", origin);
        if origin.starts_with("Microsoft.ZuneMusic") {
            source = Players::GrooveMusic
        } else if origin == "Chrome" {
            source = Players::GenericMusicPlayer
        } else if origin.contains("Spotify") {
            source = Players::Spotify
        } else if origin == "app.ytmd" {
            source = Players::YoutubeMusic
        }
    }

    let song_metadata = info
        .get()
        .expect("Failed to unwrap song metadata even if retrieval was successful.");

    let song = Song {
        track: song_metadata
            .Title()
            .expect("Failed to retrieve title from song")
            .to_string(),
        artist: song_metadata
            .Artist()
            .expect("Failed to retrieve artist from song")
            .to_string(),
        album_art: "".to_string(),
        artist_mbid: "".to_string(),
        duration: duration,
        start_time: SystemTime::now(),
        album: song_metadata
            .AlbumTitle()
            .expect("Failed to retrieve album from song")
            .to_string(),
        is_repeat: false,
        is_playing: is_playing,
        mbid: "".to_string(),
        position: position,
        scrobble: false, // will be set later
        source: source,
        url: "".to_string(),
    };

    trace!("{:?}", song);
    trace!("Is playing?: {}", is_playing);

    return Ok(song);
}
