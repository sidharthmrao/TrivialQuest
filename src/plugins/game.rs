pub mod config;
pub mod entities;
pub mod level;
pub mod objects;

use self::entities::player::PLAYER_JUMP;

use super::physics::{movement::CollisionEvent, PhysicsObject};
use crate::plugins::{
    game::{
        config::{BACKGROUND_COLOR, PLAYER_HORIZONTAL_MOVEMENT_SPEED},
        entities::player::Player,
        level::{load_objects, LevelFile}
    },
    physics::Movable,
    render::CameraZoom
};
use bevy::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Strength(pub u32);

fn setup_game() {}

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
    if query.iter().count() != 1 {
        return;
    }

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
        app.insert_resource(CameraZoom { zoom: 0.5 });
        app.insert_resource(LevelFile {
            path: "assets/levels/level_0.xml".into()
        });
        app.add_systems(Startup, setup_game);
        app.add_systems(Startup, load_objects);
        app.add_systems(Update, move_player_left_right);
        app.add_systems(Update, handle_player_space);
    }
}
