use std::rc::Rc;

use libloading::Library;

use plugins_core::speaker::Speaker;

pub struct SpeakerProxy {
    pub speaker: Box<dyn Speaker>,
    pub _lib: Rc<Library>,
}

impl Speaker for SpeakerProxy {
    fn speak(&self) -> String {
        self.speaker.speak()
    }
}