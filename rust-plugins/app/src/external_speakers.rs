use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::rc::Rc;

use libloading::Library;

use plugins_core::plugin_declaration::PluginDeclaration;
use plugins_core::speaker::Speaker;

use crate::plugin_registrar::PluginRegistrar;
use crate::speaker_proxy::SpeakerProxy;

#[derive(Default)]
pub struct ExternalSpeakers {
    speakers: HashMap<String, SpeakerProxy>,
    libraries: Vec<Rc<Library>>,
}

impl ExternalSpeakers {
    pub fn call(&self, speaker: &str) -> String {
        self.speakers.get(speaker)
            .ok_or_else(|| format!("{} not found", speaker))
            .unwrap()
            .speak()
    }

    pub fn list(&self) {
        self.speakers.iter()
            .for_each(|(speaker_name, _)| println!("{}", speaker_name))
    }

    pub fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        unsafe {
            let library = Rc::new(Library::new(library_path)?);
            let plugin_declaration = library
                .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
                .read();

            if plugin_declaration.rustc_version != plugins_core::versions::RUSTC_VERSION
                || plugin_declaration.core_version != plugins_core::versions::CORE_VERSION {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Version mismatch",
                ));
            }

            let mut registrar = PluginRegistrar::new(Rc::clone(&library));
            (plugin_declaration.register)(&mut registrar);

            self.speakers.extend(registrar.speaker);
            self.libraries.push(library);

            Ok(())
        }
    }
}