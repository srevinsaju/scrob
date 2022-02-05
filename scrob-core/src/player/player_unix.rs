use std::time::SystemTime;

use log::trace;
use mpris::{PlaybackStatus, PlayerFinder};
use types::{integrations::Players, song::Song};

/// use linux's mpris dbus data to get the current playing
/// song from the active player. If the song is originating from
/// a dbus id containing "chromium", preference is given to
/// Plasma Browser Integration since it gives accurate values
/// on the artist, album name, and track name .
pub fn get_current_song() -> Result<Song, &'static str> {
    let player_instance = PlayerFinder::new();
    if player_instance.is_err() {
        return Err("Could not connect to DBUS");
    }
    let player_instance = player_instance.unwrap();

    let players = player_instance.find_all();
    if players.is_err() {
        return Err("Could not find any players");
    }
    let players = players.unwrap();

    let mut current_player = player_instance
        .find_active()
        .expect("Could not find active player");

    trace!("Active player: {}", current_player.bus_name());

    for player in players {
        // give preference to plasma browser integration
        // since that gives the right artist + title description
        // do this only if the current player has chromium in the bundle id
        if current_player.bus_name().contains("chromium")
            && player
                .bus_name()
                .starts_with("org.mpris.MediaPlayer2.plasma-browser-integration")
            && player.is_running()
        {
            current_player = player;
            break;
        }
        if player.is_running()
            && player
                .get_playback_status()
                .unwrap_or(PlaybackStatus::Paused)
                == PlaybackStatus::Playing
        {
            // we found a running player
            // and its playing, so this is probably the active player.
            current_player = player;
        }
    }
    let metadata = current_player
        .get_metadata()
        .expect("Could not get metadata");

    let playback_status = current_player
        .get_playback_status()
        .unwrap_or(mpris::PlaybackStatus::Playing);
    trace!("Selected current player {}", current_player.bus_name());
    let playback_status_fmt = match { playback_status } {
        mpris::PlaybackStatus::Playing => true,
        mpris::PlaybackStatus::Paused => false,
        mpris::PlaybackStatus::Stopped => false,
    };

    trace!("Raw Dbus metadata: {:?}", metadata);

    let mut artist = "";
    if let Some(hmm) = metadata.get("xesam:artist") {
        if let Some(a) = metadata.artists() {
            artist = a[0];
        } else if let Some(a) = hmm.as_str() {
            artist = a;
        }; // (metadata.artists().unwrap_or(&[]).join(", "));
    }
    // println!("{:?}", artist);

    let mut source = Players::GenericMusicPlayer;
    let url = metadata.url().unwrap_or_default().to_string();
    if url.contains("music.youtube.com") {
        source = Players::YoutubeMusic;
    } else if url.contains("youtube.com") {
        source = Players::Youtube;
    } else if url.contains("open.spotify.com") || current_player.bus_name().contains("spotify") {
        source = Players::Spotify;
    } else if url.contains("youtube.com") {
        source = Players::Youtube;
    } else if current_player
        .bus_name()
        .starts_with("org.mpris.MediaPlayer2.vlc")
    {
        source = Players::Vlc;
    } else if current_player
        .bus_name()
        .starts_with("org.mpris.MediaPlayer2.rhythmbox")
    {
        source = Players::Rhythmbox;
    } else if current_player
        .bus_name()
        .starts_with("org.mpris.MediaPlayer2.elisa")
    {
        source = Players::Elisa;
    } else if current_player
        .bus_name()
        .starts_with("org.mpris.MediaPlayer2.Lollypop")
    {
        source = Players::Lollypop;
    }

    let duration = metadata.length().unwrap_or_default();
    let album = metadata.album_name().unwrap_or_default();
    let song = Song {
        track: metadata.title().unwrap_or("").to_string(),
        artist: artist.to_string(),
        album: album.to_string(),
        album_art: metadata.art_url().unwrap_or("").to_string(),
        mbid: "".to_string(),
        artist_mbid: "".to_string(),
        position: current_player.get_position().unwrap_or_default(),
        start_time: SystemTime::now(),
        duration: duration,
        is_repeat: false, // will be set afterwards by application logic
        is_playing: playback_status_fmt,
        scrobble: false, // will be set afterwards
        url: metadata.url().unwrap_or_default().to_string(),
        source: source,
    };

    // println!("{:?}", song);
    return Ok(song);
}
