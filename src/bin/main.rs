use bevy::prelude::*;
use trivial_quest::plugins::{
    game::GamePlugin, physics::PhysicsPlugin, render::RenderPlugin
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(GamePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(RenderPlugin)
        .run();
}
