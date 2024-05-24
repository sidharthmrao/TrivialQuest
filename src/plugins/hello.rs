use crate::systems::hello::hello_world;
use crate::systems::people::{add_people, greet_people, update_people};
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people).add_systems(
            Update,
            ((update_people, greet_people).chain(), hello_world)
        );

        println!("Built HelloPlugin.");
    }
}
