use crate::plugins::game::{
    entities::{
        enemy::{Enemy, Taxonomy},
        player::Player
    },
    objects::platform::Platform
};
use bevy::prelude::*;
use crate::plugins::game::objects::platform::PlatformType;

use super::physics::{Collider, Movable};

pub mod entities;
pub mod objects;
pub mod shared;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PLAYER_SPEED: f32 = 200.0;

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

    movable.pos_mut().x += direction * PLAYER_SPEED * time.delta_seconds();
}

// pub fn create_collider_on_bounding_box_change(
//     mut commands: Commands,
//     objects: Query<(bevy::prelude::Entity, &RenderBounds), Changed<RenderBounds>>
// ) {
//     for (entity, bounds) in objects.iter() {
//         commands.entity(entity).remove::<Collider>();
//
//         commands.entity(entity).insert(Collider::AABB(Aabb2d::new(
//             Vec2 { x: 0.0, y: 0.0 },
//             Vec2 {
//                 x: bounds.0.width(),
//                 y: bounds.0.height()
//             }
//         )));
//     }
// }

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR));
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, move_player);
        // app.add_systems(PostUpdate, create_collider_on_bounding_box_change);
    }
}
