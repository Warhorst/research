use std::collections::HashMap;
use std::rc::Rc;

use libloading::Library;

use plugins_core::registrar::Registrar;
use plugins_core::speaker::Speaker;

use crate::speaker_proxy::SpeakerProxy;

pub struct PluginRegistrar {
    pub speaker: HashMap<String, SpeakerProxy>,
    pub lib: Rc<Library>,
}

impl PluginRegistrar {
    pub fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            speaker: HashMap::default(),
        }
    }
}

impl Registrar for PluginRegistrar {
    fn register_speaker(&mut self, name: &str, speaker: Box<dyn Speaker>) {
        let proxy = SpeakerProxy {
            speaker,
            _lib: Rc::clone(&self.lib),
        };
        self.speaker.insert(name.to_string(), proxy);
    }
}