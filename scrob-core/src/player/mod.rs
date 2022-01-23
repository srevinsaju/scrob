#[cfg(unix)]
mod player_unix;

#[cfg(windows)]
mod player_windows;

#[cfg(unix)]
pub use player_unix::get_current_song;

#[cfg(windows)]
pub use player_windows::get_current_song;
