use crate::plugins::game::{
    entities::{
        enemy::{Enemy, Taxonomy},
        player::Player
    },
    objects::platform::Platform
};
use bevy::prelude::*;
use crate::plugins::game::objects::platform::PlatformType;
use crate::plugins::game::config::{BACKGROUND_COLOR, PLAYER_HORIZONTAL_MOVEMENT_SPEED, SpritePaths};
use crate::plugins::render::Asset;

use super::physics::{Collider, Movable};

pub mod entities;
pub mod objects;
pub mod config;

pub struct GamePlugin;

#[derive(Component)]
pub struct Entity;

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
        Vec2::new(0.0, 0.0)
    );

    // Make enemy
    Enemy::spawn(&mut commands, Taxonomy::Human, None, Vec2::new(-20.0, 80.0), Vec2::new(0.0, 0.0));

    Platform::spawn(
        &mut commands,
        Vec2::new(0.0, -100.0),
        PlatformType::Grass
    );

    Platform::spawn(
        &mut commands,
        Vec2::new(18.0, -100.0),
        PlatformType::Grass
    );
}

/// Moves the player left or right when the arrow keys are pressed.
pub fn move_player(
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
    if keyboard_input.pressed(KeyCode::Space) {
        movable.vel_mut().y += 1000.0 * time.delta_seconds();
    }

    movable.pos_mut().x += direction * PLAYER_HORIZONTAL_MOVEMENT_SPEED * time.delta_seconds();
}

pub fn update_collider(
    mut objects: Query<(&Asset, &mut Collider), Changed<Asset>>
) {
    for (sprite, mut collider) in objects.iter_mut() {
        collider.set_size(sprite.size);
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR));
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, move_player);
        app.add_systems(PostUpdate, update_collider);
    }
}
