use crate::{
    plugins::physics::Velocity,
    sprites::{
        entities::{
            enemy::{spawn_enemy, Taxonomy},
            player::{spawn_player, Player}
        },
        objects::platform::Platform
    }
};
use bevy::prelude::*;

use super::render::MainCamera;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const PLAYER_COLOR: Color = Color::rgb(0.0, 0.9, 0.9);

pub const PLAYER_SPEED: f32 = 200.0;

pub const PLATFORM_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub struct GamePlugin;

fn setup_game(mut commands: Commands) {
    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: PLATFORM_COLOR,
    //             ..default()
    //         },
    //         transform: Transform {
    //             translation: Vec2::new(0.0, 0.0).extend(0.0),
    //             scale: Vec3::new(100.0, 100.0, 1.0),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Platform
    // ));

    // Make player
    // spawn_player(
    //     &mut commands,
    //     "Player".to_string(),
    //     100,
    //     10,
    //     Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     Velocity(Vec2::new(0.0, 0.0))
    // );

    spawn_enemy(
        &mut commands,
        Taxonomy::Human,
        None,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Velocity(Vec2::new(0.0, 0.0))
    );
}

/// Moves the player left or right when the arrow keys are pressed.
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>, time: Res<Time>
) {
    if query.iter().count() != 1 {
        return;
    }

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
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>
) {
    if player_query.iter().count() != 1 {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation = player_transform.translation;
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR));
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, move_player);
        app.add_systems(Update, camera_follow_player);
    }
}
