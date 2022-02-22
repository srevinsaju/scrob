#![windows_subsystem = "console"]

use clap::clap_app;
use config as meta;
use log::trace;

use dialoguer::{theme::ColorfulTheme, Input, Password};
use env_logger;
use scrob_core::core::core;
use scrob_core::Preferences;
use types::config::ScrobConfig;

fn config_unix() {
    let cfg: ScrobConfig = confy::load("scrob")
        .expect("Error loading config. Have you logged in yet? Login with 'login' subcommand");
    todo!("Not implemented yet");
}

fn main() {
    env_logger::init();

    let matches = clap_app!(scrob =>
        (version: meta::APP_VERSION)
        (author: "Srevin Saju <lyrix+rs@srev.in>")
        (about: "Open Source Music Network")
        (@arg disable_discord: --("no-discord") "Disable discord integration")
        (@arg disable_scrobble: --("no-scrobble") "Disable scrobble support")
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
        // read until a newline (but not including it)
        let username: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("last.fm username")
            .interact_text()
            .unwrap();

        let password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("password")
            .interact()
            .unwrap();

        println!("Login details saved to config folder");

        let cfg = ScrobConfig {
            version: 1,
            password: password,
            username: username,
            ..Default::default()
        };

        confy::store("scrob", &cfg).expect("Failed to store config");
        return;
    }

    if let Some(_) = matches.subcommand_matches("config") {
        #[cfg(unix)]
        config_unix();

        return;
    }

    let prefs = Preferences {
        disable_discord_rich_presence: matches.is_present("disable_discord"),
        disable_lastfm_scrobble: matches.is_present("disable_scrobble"),
    };
    trace!("Preferences: {:?}", prefs);

    if let Some(_) = matches.subcommand_matches("gui") {
        //gui::main::main_gui();
    } else {
        core(prefs);
    }
}
