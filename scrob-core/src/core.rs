use log::{debug, info, trace, warn};


use types::config::ScrobConfig;
use types::song::Song;
use config as meta;
use colored::*;

use crate::Context;
use crate::Preferences;
use crate::integrations::base::BaseIntegrationTrait;
use crate::integrations::discord::Discord;
use crate::integrations::lastfm::Lastfm;
use crate::integrations::notification::Notification;
use crate::player;


const INTERVAL: u64 = 2000;

/// `main_loop` handles two kinds of events, song playback 
/// and song pause. When the song is played, or a song is changed
/// the .set event is triggered, and when the song is stopped 
/// the .release event is triggered.
/// This is repeated for every plugin which is enabled by the command 
/// line arguments or the context provided as the first argument `Context`
pub fn main_loop(ctx: Context) {

    let mut current_song = Song::new();

    let mut integrations = ctx.integrations;

    debug!("Starting main loop");
    loop {
        // main loop
        trace!("Checking for new song");
        let res = player::get_current_song();
        if let Err(e) = res {
            warn!(
                "Error when trying to fetch the current song from player: {}",
                e
            );
            continue;
        }
        let mut res = res.unwrap();

        trace!("Received pre-parsed song {:?}", res);

        res.scrobble = ctx.preferences.enable_lastfm_scrobble;

        if res.track == "" || !res.is_playing {
            for i in integrations.iter_mut() {
                if !i.enabled() {
                    continue;
                }
                if let Err(err) = i.release(current_song.clone()) {
                    warn!(
                        "Error when trying to clear song in integration '{}': {}",
                        i.name(),
                        err
                    );
                }
            }
        }

        if res.is_playing {
            if current_song.track != res.track || current_song.artist != res.artist || current_song.is_playing != res.is_playing {

                info!(".set triggered for {}", res.track);
                // the song has changed or the song was paused previously, but now started playing
                for i in integrations.iter_mut() {
                    if let Err(err) = i.set(res.clone(), current_song.clone()) {
                        warn!(
                            "Error when trying to update song in integration '{}': {}",
                            i.name(),
                            err
                        );
                    };
                }
                current_song.track = res.track.clone();
                current_song.artist = res.artist.clone();
                current_song.album = res.album.clone();
                println!("{}\n{}\n{}\n\n", current_song.track.green(), current_song.artist, current_song.album.bold());

            } else {
                // the +3 is to accomodate for latencies on position report
                let is_repeat =
                    current_song.position > res.position && current_song.track == res.track;
                trace!("The song is on repeat?: {}", is_repeat);

                // the song has not changed
                if is_repeat {
                    trace!("The song is on repeat!!, {:?} {:?}", current_song, res);
                    println!(
                        "{}\n{}\n{}\non Repeat.\n\n",
                        current_song.track.green(), current_song.artist, current_song.album.bold(),
                    );
                    res.is_repeat = true;
                    for i in integrations.iter_mut() {
                        if !i.enabled() {
                            continue;
                        }
                        if let Err(err) = i.set(res.clone(), current_song.clone()) {
                            warn!(
                                "Error when trying to update song in integration '{}': {}",
                                i.name(),
                                err
                            );
                        };
                    }
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

    let mut integrations: Vec<Box<dyn BaseIntegrationTrait>> = Vec::new();


    let notifs = Notification::new().expect("Failed to instantiate notification wrapper");
    integrations.push(Box::new(notifs));


    if prefs.enable_discord_rich_presence {
        info!("Connecting to discord...");
        let ds = Discord::new().unwrap();
        integrations.push(Box::new(ds));
    }

    if prefs.enable_lastfm_scrobble {
        info!("Connecting to last.fm...");
        let cfg = cfg.clone();
        let lastfm = Lastfm::new(cfg.username, cfg.password).unwrap();
        integrations.push(Box::new(lastfm));
    }

    let ctx: Context = Context {
        integrations: integrations,
        config: cfg,
        preferences: prefs,
    };

    info!("Listening to songs!");
    main_loop(ctx);
}
