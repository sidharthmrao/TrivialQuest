use crate::components::name::Name;
use crate::components::person::Person;
use bevy::prelude::*;

#[derive(Resource)]
pub struct UpdateTimer(pub Timer);

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

pub fn update_people(
    time: Res<Time>, mut timer: ResMut<UpdateTimer>,
    mut query: Query<&mut Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut name in &mut query {
            match name.0.as_str() {
                "Alice" => name.0 = "Alice Malice".to_string(),
                "Bob" => name.0 = "Bobby".to_string(),
                "Alice Malice" => name.0 = "Alice".to_string(),
                _ => {}
            }
        }
    }
}

pub fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
