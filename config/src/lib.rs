pub const APP_NAME: &str = "scrob";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_URL: &str = "https://github.com/srevinsaju/scrob";
pub const DISCORD_APPID_GENERIC: &str = "934740230538407968";
pub const DISCORD_APPID_YOUTUBE_MUSIC: &str = "916058337152499763";
pub const DISCORD_APPID_YOUTUBE: &str = "944684385372884994";
pub const DISCORD_APPID_SPOTIFY: &str = "916059119415328788";
pub const DISCORD_APPID_LOLLYPOP: &str = "939527592552308786";
pub const DISCORD_APPID_ELISA: &str = "917359677090721842";
pub const DISCORD_APPID_GROOVE_MUSIC: &str = "916059531879006238";

pub const LASTFM_APP_ID: &str = "490808bbb019f2662806d3cb24e07be6";
pub const LASTFM_APP_SECRET: &str = "f55435563994f11a6cd8c33aaf8ceb79";

pub fn discord_app_id(app_id: String) -> &'static str {
    let app_id = app_id.as_str();
    match app_id {
        "elisa" => DISCORD_APPID_ELISA,
        "youtube-music" => DISCORD_APPID_YOUTUBE_MUSIC,
        "spotify" => DISCORD_APPID_SPOTIFY,
        "youtube" => DISCORD_APPID_YOUTUBE,
        "groove-music" => DISCORD_APPID_GROOVE_MUSIC,
        "lollypop" => DISCORD_APPID_LOLLYPOP,
        _ => DISCORD_APPID_GENERIC,
    }
}
