use bevy::prelude::*;
use trivial_quest::plugins::hello::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
