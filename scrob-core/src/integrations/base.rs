use std::error::Error;

use types::song::Song;
/*
pub struct BaseIntegration {
    pub name: String, // name of the integration
}*/

pub trait BaseIntegrationTrait {
    fn set(&mut self, song: Song, last_song: Song) -> Result<(), Box<dyn Error>>;
    fn release(&mut self, last_song: Song) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> String;
    fn enabled(&self) -> bool; 
    fn set_enabled(&mut self, v: bool);
}
