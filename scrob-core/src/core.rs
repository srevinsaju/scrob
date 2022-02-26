use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::SystemTime;

use log::{debug, info, trace, warn};

use colored::*;
use config as meta;

use types::config::OperationType;
use types::config::ScrobConfig;
use types::config::ScrobMessage;
use types::integrations::Integrations;
use types::integrations::Players;
use types::song::Song;

use crate::integrations::base::BaseIntegrationTrait;
use crate::integrations::discord::Discord;
use crate::integrations::lastfm::Lastfm;
use crate::integrations::notification::Notification;
use crate::mb::search_musicbrainz;
use crate::player;
use crate::Context;
use crate::Preferences;

const INTERVAL: u64 = 2000;

/// `main_loop` handles two kinds of events, song playback
/// and song pause. When the song is played, or a song is changed
/// the .set event is triggered, and when the song is stopped
/// the .release event is triggered.
/// This is repeated for every plugin which is enabled by the command
/// line arguments or the context provided as the first argument `Context`
pub fn main_loop(ctx: Context, rx: Receiver<ScrobMessage>, song_events_sender: Sender<Song>) {
    let mut current_song = Song::new();
    let mut custom_player_set = false;
    let mut custom_player = Players::GenericMusicPlayer;

    let mut integrations = ctx.integrations;

    debug!("Starting main loop");
    loop {
        // main loop
        let mut force = false;
        trace!("Checking for new song");
        let data = rx.try_recv();
        if data.is_ok() {
            let msg = data.unwrap();
            debug!("Received message from mpsc channel {:?}", msg);
            match msg.operation.type_ {
                OperationType::Event => todo!(),
                OperationType::CustomPlayer => {
                    custom_player_set = msg.operation.enabled;
                    force = true;
                    if custom_player_set {
                        custom_player = msg.operation.custom_player;
                    } else {
                        custom_player = Players::GenericMusicPlayer;
                    }
                }
                OperationType::Integration => {
                    if integrations.contains_key(&msg.operation.integration) {
                        debug!(
                            "Toggling integration {:?} with {:?}",
                            msg.operation.integration, msg.operation.enabled
                        );

                        // set the integrationto be enabled or disabled
                        // so that on the next iteration, they take effect
                        integrations
                            .get_mut(&msg.operation.integration)
                            .unwrap()
                            .set_enabled(msg.operation.enabled);

                        if !msg.operation.enabled {
                            // disable the currently targetted integration
                            let _ = integrations
                                .get_mut(&msg.operation.integration)
                                .unwrap()
                                .release(current_song.clone());
                        } else {
                            force = true;
                        }
                    }
                }
                OperationType::Null => {}
            }
        }

        let res = player::get_current_song();

        if let Err(e) = res {
            warn!(
                "Error when trying to fetch the current song from player: {}",
                e
            );
            trace!("Sleeping for {} seconds", INTERVAL);
            std::thread::sleep(std::time::Duration::from_millis(INTERVAL));
            continue;
        }
        let mut res = res.unwrap();

        trace!("Received pre-parsed song {:?}", res);

        res.scrobble = !ctx.preferences.disable_lastfm_scrobble;

        if res.track == "" || !res.is_playing {
            for (_, v) in integrations.iter_mut() {
                if !v.enabled() {
                    continue;
                }
                if let Err(err) = v.release(current_song.clone()) {
                    warn!(
                        "Error when trying to clear song in integration '{}': {}",
                        v.name(),
                        err
                    );
                }
            }
        }

        if custom_player_set {
            res.source = custom_player;
        }

        if res.is_playing || force {
            if (current_song.track != res.track
                || current_song.artist != res.artist
                || current_song.is_playing != res.is_playing)
                || force
            {
                info!(".set triggered for {}", res.track);

                // the song needs to post processed
                let postproecessed_res = search_musicbrainz(&res);

                trace!("Received post-parsed song {:?}", postproecessed_res);

                // the song has changed or the song was paused previously, but now started playing
                for (_, v) in integrations.iter_mut() {
                    if let Err(err) = v.set(postproecessed_res.clone(), current_song.clone()) {
                        warn!(
                            "Error when trying to update song in integration '{}': {}",
                            v.name(),
                            err
                        );
                    };
                }
                trace!(
                    "Calculated duration difference {:?} - {:?}",
                    current_song.start_time,
                    postproecessed_res.start_time
                );

                current_song.track = res.track.clone();
                current_song.artist = res.artist.clone();
                current_song.album = res.album.clone();
                current_song.start_time = SystemTime::now();
                let _ = song_events_sender.send(res.clone());

                if postproecessed_res.track != current_song.track
                    || postproecessed_res.artist != current_song.artist
                {
                    println!(
                        "{} ~> {}\n{} ~> {}\n{} ~> {}\n\n",
                        current_song.track.bright_black(),
                        postproecessed_res.track.green(),
                        current_song.artist.bright_black(),
                        postproecessed_res.artist,
                        current_song.album.bright_black(),
                        postproecessed_res.album.bold()
                    );
                } else {
                    println!(
                        "{}\n{}\n{}\n\n",
                        current_song.track.green(),
                        current_song.artist,
                        current_song.album.bold()
                    );
                }
            } else {
                // the +3 is to accomodate for latencies on position report
                let is_repeat = current_song.position > res.position
                    && current_song.track == res.track
                    && res.artist == current_song.artist;
                trace!("The song is on repeat?: {}", is_repeat);

                // the song has not changed
                if is_repeat {
                    trace!("The song is on repeat!!, {:?} {:?}", current_song, res);
                    let _ = song_events_sender.send(res.clone());
                    println!(
                        "{}\n{}\n{}\non Repeat.\n\n",
                        current_song.track.green(),
                        current_song.artist,
                        current_song.album.bold(),
                    );
                    res.is_repeat = true;
                    for (_, v) in integrations.iter_mut() {
                        if !v.enabled() {
                            continue;
                        }
                        if let Err(err) = v.set(res.clone(), current_song.clone()) {
                            warn!(
                                "Error when trying to update song in integration '{}': {}",
                                v.name(),
                                err
                            );
                        };
                    }
                    current_song.start_time = SystemTime::now();
                };
            };
        }

        current_song.position = res.position;
        current_song.is_playing = res.is_playing;

        trace!("Sleeping for {} seconds", INTERVAL);
        std::thread::sleep(std::time::Duration::from_millis(INTERVAL));
    }
}

/// loads all the configuration and parses the preferences.
/// runs the `main_loop` at the end of the function.
pub fn core(prefs: Preferences) {
    info!("{} {}", meta::APP_NAME, meta::APP_VERSION);

    ctrlc::set_handler(move || {
        info!("Received control + c, Cleaning up gracefully...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let cfg: ScrobConfig = confy::load("scrob")
        .expect("Error loading config. Have you logged in yet? Login with 'login' subcommand");

    debug!("Connecting to api...");

    let mut integrations: HashMap<Integrations, Box<dyn BaseIntegrationTrait>> = HashMap::new();

    let notifs = Notification::new().expect("Failed to instantiate notification wrapper");
    integrations.insert(Integrations::Notification, Box::new(notifs));

    if !prefs.disable_discord_rich_presence {
        let cfg = cfg.clone();
        info!("Connecting to discord...");
        let ds = Discord::new(cfg.discord).unwrap();
        integrations.insert(Integrations::Discord, Box::new(ds));
    }

    if !prefs.disable_lastfm_scrobble {
        info!("Connecting to last.fm...");
        let cfg = cfg.clone();
        let lastfm = Lastfm::new(cfg.lastfm).unwrap();
        integrations.insert(Integrations::Lastfm, Box::new(lastfm));
    }

    let ctx: Context = Context {
        integrations: integrations,
        config: cfg,
        preferences: prefs,
    };

    let (_, rx) = channel();
    let (song_events_sender, _) = channel();
    info!("Listening to songs! 🎶");
    main_loop(ctx, rx, song_events_sender);
}
