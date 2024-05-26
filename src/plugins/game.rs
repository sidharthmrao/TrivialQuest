pub mod config;
pub mod entities;
pub mod objects;

use self::entities::player::PLAYER_JUMP;

use super::physics::{
    collider::Hitbox, movement::CollisionEvent, PhysicsObject
};
use crate::plugins::{
    game::{
        config::{BACKGROUND_COLOR, PLAYER_HORIZONTAL_MOVEMENT_SPEED},
        entities::{
            enemy::{Enemy, Taxonomy},
            player::Player
        },
        objects::platform::{Platform, PlatformType}
    },
    physics::Movable,
    render::{Asset}
};
use bevy::prelude::*;
use crate::plugins::render::CameraZoom;

pub struct GamePlugin;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Strength(pub u32);

fn setup_game(mut commands: Commands) {
    // Make player
    Player::spawn(
        &mut commands,
        "Player".into(),
        100,
        10,
        Vec2::new(20.0, 80.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 0.0)
    );

    // Make enemy
    Enemy::spawn(
        &mut commands,
        Taxonomy::Human,
        None,
        Vec2::new(-20.0, 80.0),
        Vec2::new(1., 1.),
        Vec2::new(0.0, 0.0)
    );

    Platform::spawn(
        &mut commands,
        Vec2::new(0.0, -100.0),
        PlatformType::Grass,
        Vec2::new(1.0, 1.0)
    );

    Platform::spawn(
        &mut commands,
        Vec2::new(18.0, -100.0),
        PlatformType::Grass,
        Vec2::new(1.0, 1.0)
    );

    Platform::spawn(
        &mut commands,
        Vec2::new(-18.0, -118.0),
        PlatformType::Grass,
        Vec2::new(1.0, 1.0)
    );

    Platform::spawn(
        &mut commands,
        Vec2::new(36.0, -118.0),
        PlatformType::Grass,
        Vec2::new(1.0, 1.0)
    );

    commands.insert_resource(CameraZoom { zoom: 0.5 });
}

/// Moves the player left or right when the arrow keys are pressed.
pub fn move_player_left_right(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Movable, With<Player>>, time: Res<Time>
) {
    if query.iter().count() != 1 {
        return;
    }

    let mut movable = query.single_mut();

    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    movable.pos_mut().x +=
        direction * PLAYER_HORIZONTAL_MOVEMENT_SPEED * time.delta_seconds();
}

pub fn handle_player_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Movable, &PhysicsObject), With<Player>>,
    time: Res<Time>
) {
    let (mut movable, player) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Space) {
        for event in collision_events.read() {
            if event.movable == *player
                && event.fixed.matches("platform".into())
            {
                movable.vel_mut().y = PLAYER_JUMP * time.delta_seconds();
            }
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR));
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, move_player_left_right);
        app.add_systems(Update, handle_player_space);
    }
}
