use crate::components::name::Name;
use crate::components::person::Person;
use bevy::prelude::*;

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        match name.0.as_str() {
            "Alice" => name.0 = "Alice Malice".to_string(),
            "Bob" => name.0 = "Bobby".to_string(),
            _ => {}
        }
    }
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
