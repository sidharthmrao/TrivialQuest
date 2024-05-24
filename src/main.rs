use bevy::prelude::*;
use trivial_quest::components::name::Name;
use trivial_quest::components::person::Person;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, ((update_people, greet_people).chain(), hello_world))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alice" {
            name.0 = "Alice Balice".to_string();
            break;
        }
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn hello_world() {
    println!("Hello");
}
