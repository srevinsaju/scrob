use types::{integrations::Players, song::Song};

use crate::integrations::base::BaseIntegrationTrait;

use config as meta;
use discord_rich_presence::{activity, new_client, DiscordIpc};
use log::{trace, warn};
use std::{error::Error, time::UNIX_EPOCH};

pub struct Discord {
    pub application_id: String,
    ds: Box<dyn DiscordIpc>,
    pub enabled: bool,
    pub connected: bool,
    last_source: Players,
}

impl Discord {
    fn safe_discord(app_id: &'static str) -> Box<dyn DiscordIpc> {
        let discord = new_client(app_id).expect("Failed to create discord client.");
        return Box::new(discord);
    }

    pub fn new() -> Result<Discord, &'static str> {
        // handle error here

        let discord = Discord::safe_discord(meta::DISCORD_APPID_GENERIC);

        Ok(Discord {
            application_id: meta::DISCORD_APPID_GENERIC.to_string(),
            ds: discord,
            enabled: true,
            connected: false,
            last_source: Players::GenericMusicPlayer,
        })
    }
}

// handles discord integrations and playback activity status
impl BaseIntegrationTrait for Discord {
    fn set(&mut self, song: Song, _: Song) -> Result<(), Box<dyn Error>> {
        if song.source == Players::Spotify {
            return Ok(()); // spotify has their own discord integration
        }
        trace!("setting discord integration");
        if !self.connected || self.last_source != song.source {
            // that means we are creating discord rich presence now
            // or either the song source has change, for example youtube music to spotify, etc.
            self.ds = Discord::safe_discord(song.source.as_discord_app_id());
            self.ds.connect()?;
            self.last_source = song.source.clone();
            self.connected = true;
        }

        let mut info = song.track;
        if song.is_repeat {
            info += " - on Repeat";
        }

        let app_desc = format!("{} {}", meta::APP_NAME, meta::APP_VERSION);
        trace!("setting discord activity with image '{}' and app desc '{}'", song.source.as_str(), app_desc);
        let assets = activity::Assets::new()
            .large_image(song.source.as_str())
            .large_text(app_desc.as_str())
            .small_text(song.source.as_str())
            .small_image("lyrix-music");

        let current_time = std::time::SystemTime::now();
        let since_the_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        #[cfg(windows)]
        let timestamps = activity::Timestamps::new().start(since_the_epoch.as_secs() as i64);

        #[cfg(unix)]
        let mut end_timestamp =
            (since_the_epoch.as_secs() + song.duration.as_secs() - song.position.as_secs()) as i64;
        #[cfg(unix)]
        if end_timestamp < since_the_epoch.as_secs() as i64 {
            // if the song is over, set the end timestamp to
            // the current time, so discord will show the song as over
            end_timestamp = since_the_epoch.as_secs() as i64;
        }
        #[cfg(unix)]
        let timestamps = activity::Timestamps::new()
            //.start( as i64)
            .end(end_timestamp);

        let mut artist = song.artist.clone();
        if artist == "" {
            artist = "Playing".to_string();
        }

        let res = self.ds.set_activity(
            activity::Activity::new()
                .state(artist.as_str())
                .details(info.as_str())
                .assets(assets)
                .timestamps(timestamps),
        );
        if let Err(res) = res {
            warn!("{}", res);
        }
        Ok(())
    }

    fn release(&mut self, _: Song) -> Result<(), Box<dyn Error>> {
        if !self.connected {
            return Ok(());
        }
        trace!("releasing discord integration");
        self.ds.close()?;
        self.connected = false;

        Ok(())
    }

    fn name(&self) -> String {
        "Discord".to_string()
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
    }
}
