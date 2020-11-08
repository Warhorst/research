use plugins_core::export_plugin;
use plugins_core::registrar::Registrar;
use plugins_core::speaker::Speaker;

pub struct HelloSpeaker;

impl Speaker for HelloSpeaker {
    fn speak(&self) -> String {
        String::from("Hello there!")
        // General Kenobi!
    }
}

export_plugin!(register_hello, "hello", create_speaker);

extern "C" fn create_speaker() -> Box<dyn Speaker> {
    Box::new(HelloSpeaker)
}

extern "C" fn register_hello(registrar: &mut dyn Registrar) {
    registrar.register_speaker("hello", Box::new(HelloSpeaker))
}


