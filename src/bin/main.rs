use bevy::prelude::*;
use trivial_quest::{
    defines::BACKGROUND_COLOR,
    systems::{apply_gravity, camera_follow_player, move_player, setup}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, apply_gravity, camera_follow_player).chain()
        )
        .run();
}
