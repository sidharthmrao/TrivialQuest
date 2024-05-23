use godot::prelude::*;
mod game;

struct TrivialExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TrivialExtension {}