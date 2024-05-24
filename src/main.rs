use bevy::prelude::*;
use trivial_quest::graphics::GraphicsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GraphicsPlugin))
        .run();
}
