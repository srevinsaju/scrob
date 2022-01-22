use log::{trace, debug};
use types::song::Song;

use crate::integrations::base::BaseIntegrationTrait;
use std::{error::Error, time::Duration};

use rustfm_scrobble::{Scrobble, Scrobbler};

pub struct Lastfm {
    scrobbler: Scrobbler,
    pub enabled: bool,
    
}

impl Lastfm {
    pub fn new(username: String, password: String) -> Result<Lastfm, &'static str> {
        let mut scrobbler = Scrobbler::new(config::LASTFM_APP_ID, config::LASTFM_APP_SECRET);
        scrobbler.authenticate_with_password(username.as_str(), password.as_str()).expect("Failed to authenticate with Last.fm");
        Ok(Lastfm {
            enabled: true,
            scrobbler: scrobbler
        })
        
    }
}

// sends the song to lastfm for playback
impl BaseIntegrationTrait for Lastfm {
    fn set(&mut self, song: Song, last_song: Song) -> Result<(), Box<dyn Error>> {

        trace!("Set now playing on last.fm");
        if last_song.track != song.track {
            debug!("last song: {} and current song: {}", last_song.track, song.track);    
        }
        
        if song.is_repeat || !(song.track == last_song.track && song.artist == last_song.artist) {
            debug!("Song is on repeat, or song has changed, scrobbling previous, and sending fresh scrobble");
            if last_song.duration < Duration::from_secs(80) {
                trace!("Last song wasn't longer than 80 seconds, not scrobbling");
            } else {
                let scrobble = Scrobble::new(last_song.artist.as_str(), last_song.track.as_str(), last_song.album.as_str());
                self.scrobbler.scrobble(&scrobble).expect("Couldn't send scrobble");
            }
        }
        let scrobble = Scrobble::new(song.artist.as_str(), song.track.as_str(), song.album.as_str());
        self.scrobbler.now_playing(&scrobble).expect("Failed to send song to lastfm");    
        
        

        Ok(())
    }

    fn release(&mut self, last_song: Song) -> Result<(), Box<dyn Error>> {

        if last_song.duration > Duration::from_secs(80) {
            debug!("Scrobbling track which was being played before released");
            let scrobble = Scrobble::new(last_song.artist.as_str(), last_song.track.as_str(), last_song.album.as_str());
            self.scrobbler.scrobble(&scrobble).expect("Couldn't send scrobble");
        } else {
            trace!("Not scrobbling because stopped before 80 seconds was completed");
        }
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
