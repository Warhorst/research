pub mod speaker;
pub mod plugin_declaration;
pub mod registrar;
pub mod versions;

#[macro_export]
macro_rules! export_plugin {
    ($register:expr, $name:expr, $create_speaker:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::plugin_declaration::PluginDeclaration = $crate::plugin_declaration::PluginDeclaration {
            rustc_version: $crate::versions::RUSTC_VERSION,
            core_version: $crate::versions::CORE_VERSION,
            plugin_name: $name,
            speaker: $create_speaker,
            register: $register,
        };
    };
}
