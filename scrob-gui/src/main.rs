use log::{debug, info};
use scrob_core::Preferences;
use scrob_core::{
    core::main_loop,
    integrations::{
        base::BaseIntegrationTrait, discord::Discord, lastfm::Lastfm, notification::Notification,
    },
    Context,
};
use std::thread::spawn;
use types::config::{ScrobMessage, ScrobOperation};
use types::integrations::Players;
use types::song::Song;
use wry::application::dpi::LogicalSize;
use wry::http::ResponseBuilder;

use std::{collections::HashMap, sync::mpsc::channel, thread};
use types::{config::ScrobConfig, integrations::Integrations};
use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder, WindowId},
    },
    webview::WebViewBuilder,
};

static BUNDLE_JS: &'static [u8] = include_bytes!("../../web/dist/js/bundle.js");
static STATIC_CSS_SCROB: &'static [u8] = include_bytes!("../../web/dist/static/css/common.css");
static STATIC_CSS_COMMON: &'static [u8] = include_bytes!("../../web/dist/static/css/lyrix.css");
static SCROB_LOGO: &'static [u8] = include_bytes!("../../web/dist/static/lyrix-desktop.png");

fn main() {
    let index_html = include_str!("../../web/dist/index.html");

    enum UserEvents {
        CloseWindow(WindowId),
        NewWindow(),
    }

    env_logger::init();

    let (song_events_sender, song_events_receiver) = channel::<Song>();

    let socket = ws::WebSocket::new(|_| {
        move |_| panic!("This server cannot receive messages, it only sends them.")
    })
    .expect("Unable to create WebSocket");

    // Get a sender for ALL connections to the websocket
    let broacaster = socket.broadcaster();

    // Setup thread for listening to the channel and broadcasting the latest data to
    // all currently connected clients
    let broad = thread::spawn(move || loop {
        if let Ok(song) = song_events_receiver.recv() {
            debug!("websocket sending {:?}", song);
            broacaster
                .send(serde_json::to_string(&song).expect("Couldn't parse song to json"))
                .expect("Unable to send WebSocket message.")
        } else {
            info!("Shutting down broadcaster thread.");
            break;
        }
    });

    spawn(move || {
        // Run the websocket
        socket
            .listen("127.0.0.1:3012")
            .expect("Unable to run WebSocket.");
        broad.join().expect("Broadcaster thread failed.");
    });

    let event_loop = EventLoop::<UserEvents>::with_user_event();

    let proxy = event_loop.create_proxy();

    let (tx, rx) = channel();
    let handler = move |window: &Window, req: String| match req.as_str() {
        "new-window" => {
            let _ = proxy.send_event(UserEvents::NewWindow());
        }
        "close" => {
            let _ = proxy.send_event(UserEvents::CloseWindow(window.id()));
        }
        "scrobbleSwitchEnabled" => {
            println!("Scrobble Enable Request Received");
            tx.send(ScrobMessage {
                operation: ScrobOperation {
                    type_: types::config::OperationType::Integration,

                    enabled: true,
                    integration: Integrations::Lastfm,
                    ..Default::default()
                },
            })
            .expect("Couldn't send to mpsc queue");
            println!("Scrobble Enable Requested");
        }
        "scrobbleSwitchDisabled" => {
            println!("Scrobble Disable Request Received");
            tx.send(ScrobMessage {
                operation: ScrobOperation {
                    type_: types::config::OperationType::Integration,

                    enabled: false,
                    integration: Integrations::Lastfm,
                    ..Default::default()
                },
            })
            .expect("Couldn't send to mpsc queue");
            println!("Scrobble Disable Requested");
        }
        "discordSwitchEnabled" => {
            println!("Discord Enable Request Received");
            tx.send(ScrobMessage {
                operation: ScrobOperation {
                    type_: types::config::OperationType::Integration,

                    enabled: true,
                    integration: Integrations::Discord,
                    ..Default::default()
                },
            })
            .expect("Couldn't send to mpsc queue");
            println!("Discord Enable Requested");
        }
        "discordSwitchDisabled" => {
            println!("Discord Disable Request Received");
            tx.send(ScrobMessage {
                operation: ScrobOperation {
                    type_: types::config::OperationType::Integration,

                    enabled: false,
                    integration: Integrations::Discord,
                    ..Default::default()
                },
            })
            .expect("Couldn't send to mpsc queue");
            println!("Discord Disable Requested");
        }
        "playerChangeReset" => {
            let _ = tx.send(ScrobMessage {
                operation: ScrobOperation {
                    type_: types::config::OperationType::CustomPlayer,

                    enabled: false,
                    custom_player: Players::GenericMusicPlayer,
                    ..Default::default()
                },
            });
        }

        _ if req.starts_with("playerChangeRequested") => {
            let new_player_name = req.replace("playerChangeRequested:", "");
            let new_player_name = new_player_name.as_str();
            let player = Players::new(new_player_name);
            let _ = tx.send(ScrobMessage {
                operation: ScrobOperation {
                    type_: types::config::OperationType::CustomPlayer,

                    enabled: true,
                    custom_player: player,
                    ..Default::default()
                },
            });
        }

        _ => (),
    };

    let window = WindowBuilder::new()
        .with_title("Scrob")
        .with_inner_size(LogicalSize::<i32>::new(400, 410))
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_custom_protocol("wry".into(), move |request| {
            let path = request.uri().replace("wry://", "");
            let (data, meta) = match path.as_str() {
                "/static/lyrix-desktop.png" => (SCROB_LOGO, "image/png"),
                "/static/css/common.css" => (STATIC_CSS_COMMON, "text/css"),
                "/static/css/lyrix.css" => (STATIC_CSS_SCROB, "text/css"),
                "/js/bundle.js" => (BUNDLE_JS, "text/javascript"),
                _ => unimplemented!(),
            };

            ResponseBuilder::new().mimetype(meta).body(data.to_vec())
        })
        .with_dev_tool(true)
        .with_html(index_html)
        .unwrap()
        .with_ipc_handler(handler)
        .build()
        .unwrap();

    thread::spawn(move || {
        let cfg: ScrobConfig = confy::load("scrob")
            .expect("Error loading config. Have you logged in yet? Login with 'login' subcommand");

        let mut integrations: HashMap<Integrations, Box<dyn BaseIntegrationTrait>> = HashMap::new();

        let notifs = Notification::new().expect("Failed to instantiate notification wrapper");
        integrations.insert(Integrations::Notification, Box::new(notifs));

        let prefs = Preferences {
            disable_discord_rich_presence: false,
            disable_lastfm_scrobble: false,
        };

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
        main_loop(ctx, rx, song_events_sender);
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event,
                window_id: _,
                ..
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(_) => {
                    let _ = _webview.resize();
                }
                _ => (),
            },
            _ => (),
        }
    });
}
