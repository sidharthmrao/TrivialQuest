use super::components::Player;
use crate::{
    components::Platform,
    defines::{PLATFORM_COLOR, PLAYER_COLOR, PLAYER_SIZE, PLAYER_SPEED},
    plugins::physics::{Gravity, Velocity}
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));

    commands.spawn((
        Gravity,
        Velocity(Vec2::ZERO),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::IDENTITY
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: PLAYER_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        },
        Player,
        Velocity::ZERO,
        Gravity
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PLATFORM_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec2::new(0.0, 0.0).extend(0.0),
                scale: Vec3::new(100.0, 100.0, 1.0),
                ..default()
            },
            ..default()
        },
        Platform
    ));
}

/// Moves the player left or right when the arrow keys are pressed.
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>, time: Res<Time>
) {
    let mut transform = query.single_mut();

    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    transform.translation.x += direction * PLAYER_SPEED * time.delta_seconds();
}

/// Locates the camera at the player.
pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation = player_transform.translation;
}
