use bevy::prelude::*;
use trivial_quest::systems::{hello::*, people::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            ((update_people, greet_people).chain(), hello_world)
        )
        .run();
}
