use godot::prelude::*;
pub mod game;

struct TrivialExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TrivialExtension {}
