use bevy::prelude::*;
use trivial_quest::{
    defines::BACKGROUND_COLOR,
    plugins::physics::PhysicsPlugin,
    systems::{camera_follow_player, move_player, setup}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, camera_follow_player).chain())
        .run();
}
