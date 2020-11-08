use crate::speaker::Speaker;

pub trait Registrar {
    fn register_speaker(&mut self, name: &str, speaker: Box<dyn Speaker>);
}