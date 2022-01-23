use log::{debug, trace, warn};
use types::song::Song;

use crate::integrations::base::BaseIntegrationTrait;
use std::{error::Error, time::Duration};

use rustfm_scrobble::{Scrobble, Scrobbler};

/// Lastfm integration settings
pub struct Lastfm {
    /// private scrobbler instance authenticated with username and password using the `new` method
    scrobbler: Scrobbler,

    /// tells if the lastfm integration is enabled
    pub enabled: bool,
}

impl Lastfm {
    /// creates a new Lastfm integration with a username and password and returns a new `Lastfm` instance
    pub fn new(username: String, password: String) -> Result<Lastfm, &'static str> {
        // do not use username and password here, but instead move to a different
        // authentication system
        let mut scrobbler = Scrobbler::new(config::LASTFM_APP_ID, config::LASTFM_APP_SECRET);
        scrobbler
            .authenticate_with_password(username.as_str(), password.as_str())
            .expect("Failed to authenticate with Last.fm");
        Ok(Lastfm {
            enabled: true,
            scrobbler: scrobbler,
        })
    }
}

/// handles lastfm integration and playback activity status
impl BaseIntegrationTrait for Lastfm {
    /// sets the "now playing" status for the song if the song is being played for the first time.
    /// If a song change has been detected, the last song is scrobbled, and the new song is set in
    /// "now playing" status.
    /// The last song is scrobbled only if the last song was played more than 80 seconds.
    /// This is to avoid scrobbling songs that are not really played.
    /// If the song is detected as one which was played on repeat by the `main_loop`, it is scrobbled
    /// first, and then set with "now playing" status.
    fn set(&mut self, song: Song, last_song: Song) -> Result<(), Box<dyn Error>> {
        trace!("Set now playing on last.fm");
        if last_song.track != song.track {
            debug!(
                "last song: {} and current song: {}",
                last_song.track, song.track
            );
        }

        if song.is_repeat || !(song.track == last_song.track && song.artist == last_song.artist) {
            let mut position = last_song.position;
            debug!("Song is on repeat, or song has changed, scrobbling previous, and sending fresh scrobble");
            if position == Duration::from_secs(0) {
                warn!("heck windows!!");
                // windows heck
                let diff = song
                    .start_time
                    .duration_since(last_song.start_time)
                    .expect("Couldn't calculate diff");
                warn!("Diff is {:?}", diff);
                position = diff;
            }
            if position < Duration::from_secs(80) {
                debug!(
                    "Last song wasn't longer than 80 seconds, not scrobbling {:?}",
                    last_song.position
                );
            } else {
                let mut album = last_song.album.as_str();
                if album == "unknown" {
                    album = ""
                }
                let scrobble = Scrobble::new(
                    last_song.get_first_artist().as_str(),
                    last_song.track.as_str(),
                    album,
                );
                if let Err(err) = self.scrobbler.scrobble(&scrobble) {
                    warn!(
                        "Couldn't send scrobble for the song {} - {}: {:?}",
                        last_song.get_first_artist(),
                        last_song.track,
                        err
                    );
                }
            }
        }
        let mut album = song.album.as_str();
        if album == "unknown" {
            album = ""
        }
        let scrobble = Scrobble::new(song.get_first_artist().as_str(), song.track.as_str(), album);
        if let Err(err) = self.scrobbler.now_playing(&scrobble) {
            warn!(
                "Failed to send now playing request for the song {} - {}: {:?}",
                last_song.get_first_artist(),
                last_song.track,
                err
            );
        }

        Ok(())
    }

    /// this does not do anything right now, but this is bad, because it will be called every time
    /// and we need to add a feature to scrobble the last track which was ever played.
    /// if this is not implemented, the last played song will not be scrobbled, because the `.set`
    /// method will not be called.
    fn release(&mut self, _: Song) -> Result<(), Box<dyn Error>> {
        /*

        if last_song.position > Duration::from_secs(80) {
            let mut album = last_song.album.as_str();
            if album == "unknown" {
                album = ""
            }
            debug!("Scrobbling track which was being played before released");
            let scrobble = Scrobble::new(last_song.get_first_artist().as_str(), last_song.track.as_str(), album);
            if let Err(err) = self.scrobbler.scrobble(&scrobble) {
                warn!("Couldn't send scrobble for the song {} - {}: {:?}", last_song.get_first_artist(), last_song.track, err);
            }
        } else {
            trace!("Not scrobbling because stopped before 80 seconds was completed");
        } */
        Ok(())
    }

    fn name(&self) -> String {
        "last.fm".to_string()
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
    }
}
