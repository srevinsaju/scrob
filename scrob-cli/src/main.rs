#![windows_subsystem = "console"]

use clap::clap_app;
use config as meta;
use log::trace;
use text_io::read;


use env_logger;
use scrob_core::Preferences;
use scrob_core::core::core;
use types::config::ScrobConfig;


fn main() {

    env_logger::init();

    let matches = clap_app!(scrob =>
        (version: meta::APP_VERSION)
        (author: "Srevin Saju <lyrix+rs@srev.in>")
        (about: "Open Source Music Network")
        (@arg discord: --discord "Enable discord integration")
        (@arg lastfm_scrobble: --("lastfm-scrobble") "Enable scrobble support")
        (@subcommand login =>
            (about: "Login/re-login to Lyrix")
        )
        (@subcommand gui =>
            (about: "Open scrob GUI")
        )
        
    )
    .get_matches();

    trace!("Cli: {:?}", matches);
    if let Some(_) = matches.subcommand_matches("login") {
        println!("Enter your last.fm username\n");

        // read until a newline (but not including it)
        let username: String = read!("{}\n");

        println!("Enter your last.fm password\n");
        let password: String = read!("{}\n");

        println!("Logged in successfully!\n");

        let cfg = ScrobConfig {
            version: 1,
            password: password,
            username: username,
        };

        confy::store("scrob", &cfg).expect("Failed to store config");
    }

    let prefs = Preferences {

        enable_discord_rich_presence: matches.is_present("discord"),
        enable_lastfm_scrobble: matches.is_present("lastfm_scrobble"),
    };
    trace!("Preferences: {:?}", prefs);

    if let Some(_) = matches.subcommand_matches("gui") {
        //gui::main::main_gui();
    } else {
        core(prefs);
    }
    
}
