use std::alloc::System;

use crate::external_speakers::ExternalSpeakers;

pub mod speaker_proxy;
pub mod external_speakers;
pub mod plugin_registrar;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let mut external_speakers = ExternalSpeakers::default();

    external_speakers.load("plugins/hello.dll").unwrap();

    external_speakers.list();

    println!("{}", external_speakers.call("hello"))
}