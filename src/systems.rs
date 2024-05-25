use crate::defines::{PLAYER_COLOR, PLAYER_SIZE, PLAYER_SPEED};

use super::components::Player;
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
struct GameCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
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
        Player
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>
) {
    let mut camera_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    camera_transform.translation.x +=
        -direction * PLAYER_SPEED * time.delta_seconds();
}
