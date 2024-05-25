use bevy::prelude::*;
use trivial_quest::systems;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::update)
        .run();
}
