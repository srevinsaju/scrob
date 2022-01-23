use types::song::Song;

use crate::integrations::base::BaseIntegrationTrait;
use log::warn;
use notify_rust::Notification as Ntfy;
use notify_rust::Timeout;
use std::error::Error;

pub struct Notification {
    pub enabled: bool,
}

impl Notification {
    pub fn new() -> Result<Notification, &'static str> {
        // handle error here
        Ok(Notification { enabled: true })
    }
}

// sends the song to lyrix for playback
impl BaseIntegrationTrait for Notification {
    fn set(&mut self, song: Song, _: Song) -> Result<(), Box<dyn Error>> {
        let mut is_repeat = "";
        if song.is_repeat {
            is_repeat = "(on Repeat)";
        }

        let notif = Ntfy::new()
            .summary(format!("{}", song.track).as_str())
            .body(format!("{} {}", song.artist, is_repeat).as_str())
            .timeout(Timeout::Milliseconds(6000))
            .show();

        if let Err(n) = notif {
            warn!("Error when setting notification: {}", n);
        }
        Ok(())
    }

    fn release(&mut self, _: Song) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn name(&self) -> String {
        "Notification".to_string()
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
    }
}
