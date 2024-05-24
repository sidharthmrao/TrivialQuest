use bevy::prelude::*;
use bevy::utils::info;

#[derive(Component)]
struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    info("Setting up camera.");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MainCamera
    ));
}
