use crate::registrar::Registrar;
use crate::speaker::Speaker;

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub plugin_name: &'static str,
    pub speaker: extern "C" fn() -> Box<dyn Speaker>,
    pub register: extern "C" fn(&mut dyn Registrar),
}