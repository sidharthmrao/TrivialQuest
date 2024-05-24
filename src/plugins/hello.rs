use crate::systems::people::{add_people, greet_people, GreetTimer, update_people, UpdateTimer};
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(UpdateTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people).add_systems(
            Update,
            (update_people, greet_people).chain()
        );

        println!("Built HelloPlugin.");
    }
}
