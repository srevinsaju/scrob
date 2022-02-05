use config::{APP_NAME, APP_URL, APP_VERSION};
use log::{debug, warn};
use types::song::Song;

pub fn search_musicbrainz(res: &Song) -> Song {
    if std::env::var("SCROB_ENABLE_MUSICBRAINZ").is_err() {
        return res.clone();
    }

    let app_user_agent = format!("{}/{} ({})", APP_NAME, APP_VERSION, APP_URL);
    let client = reqwest::blocking::Client::builder()
        .user_agent(app_user_agent.as_str())
        .build()
        .expect("Couldn't instantiate musicbrainz client");

    let search_string = format!("{} {}", res.track, res.artist);
    let resp = client
        .get(format!(
            "http://musicbrainz.org/ws/2/release/?query={}&fmt=json",
            search_string
        ))
        .send();

    if let Err(e) = resp {
        warn!(
            "Error when trying to fetch the current song from musicbrainz: {}",
            e
        );
        return res.to_owned();
    }
    let resp = resp.unwrap();
    if let Err(e) = resp.error_for_status_ref() {
        warn!(
            "Error when trying to fetch the current song from musicbrainz: {}",
            e
        );
        return res.to_owned();
    }

    let resp = resp.json::<serde_json::Value>();
    if let Err(e) = resp {
        warn!(
            "Error when trying to parse the current song from musicbrainz: {}",
            e
        );
        return res.to_owned();
    }
    let resp = resp.unwrap();

    let releases = resp.get("releases").unwrap().as_array().unwrap();
    if releases.len() == 0 {
        warn!("No releases found for {}", res.track);
        return res.to_owned();
    }

    let first_release = releases[0].as_object().unwrap();
    debug!("{:?}", first_release);
    let artist_credit = first_release
        .get("artist-credit")
        .unwrap()
        .as_array()
        .unwrap();
    let release_group = first_release
        .get("release-group")
        .unwrap()
        .as_object()
        .unwrap();

    if artist_credit.len() == 0 {
        warn!("No artist-credit found for {}", res.track);
        return res.to_owned();
    }
    let artist_credit = artist_credit[0].as_object().unwrap();

    let title = first_release.get("title").unwrap().as_str().unwrap();
    let artist = artist_credit.get("name").unwrap().as_str().unwrap();
    let album = release_group.get("title").unwrap().as_str().unwrap();

    debug!(
        "Musicbrainz search result: {} - {} - {}",
        title, artist, album
    );

    if title == "null" || artist == "null" || album == "null" {
        warn!("Error when trying to parse the current song from musicbrainz: no title, artist or album found");
        return res.to_owned();
    }

    let mut res = res.to_owned();

    res.track = title.to_string();
    res.artist = artist.to_string();
    res.album = album.to_string();

    return res;
}
