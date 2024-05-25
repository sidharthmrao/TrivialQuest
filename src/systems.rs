use bevy::prelude::*;

#[derive(Component)]
struct GameCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), GameCamera));
}

pub fn update() {}
