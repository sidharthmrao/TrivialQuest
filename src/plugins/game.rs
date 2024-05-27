pub mod config;
pub mod entities;
pub mod level;
pub mod objects;

// use self::entities::player::PLAYER_JUMP;

use super::physics::{movement::CollisionEvent, PhysicsObject};
use crate::plugins::{
    game::{
        entities::player::Player,
        level::{load_objects, LevelFile}
    },
    physics::Movable,
    render::CameraZoom
};
use bevy::prelude::*;
use crate::plugins::game::config::GameSettings;

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
    mut query: Query<&mut Movable, With<Player>>,
    time: Res<Time>,
    settings: Res<GameSettings>
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
        direction * settings.player_horizontal_movement_speed * time.delta_seconds();
}

#[derive(Event)]
pub struct PlayerJumpEvent;

#[derive(Resource)]
pub struct PlayerJumpable {
    pub can_jump: bool
}

pub fn handle_collision(
    mut jumpable: ResMut<PlayerJumpable>,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<&PhysicsObject, With<Player>>
) {
    for event in collision_events.read() {
        if query.iter().count() != 1 {
            return;
        }

        let player = query.single();

        if event.movable == *player
            && event.fixed.matches("platform".into())
        {
            jumpable.can_jump = true;
        }
    }
}

pub fn handle_player_jump(
    mut jumpable: ResMut<PlayerJumpable>,
    mut query: Query<&mut Movable, With<Player>>,
    mut jump_events: EventReader<PlayerJumpEvent>,
    settings: Res<GameSettings>
) {
    if query.iter().count() != 1 {
        return;
    }

    let mut movable = query.single_mut();

    for _ in jump_events.read() {
        if jumpable.can_jump {
            println!("Jumping {}", settings.player_jump_vel);
            movable.vel_mut().y = settings.player_jump_vel;
            jumpable.can_jump = false;
        }
    }
}

pub fn keymap(
    mut jump_event_writer: EventWriter<PlayerJumpEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        jump_event_writer.send(PlayerJumpEvent);
    }
}

// pub fn handle_player_space(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut collision_events: EventReader<CollisionEvent>,
//     mut query: Query<(&mut Movable, &PhysicsObject), With<Player>>,
//     time: Res<Time>
// ) {
//     if query.iter().count() != 1 {
//         return;
//     }
//
//     let (mut movable, player) = query.single_mut();
//
//     if keyboard_input.pressed(KeyCode::Space) {
//         for event in collision_events.read() {
//             if event.movable == *player
//                 && event.fixed.matches("platform".into())
//             {
//                 movable.vel_mut().y = PLAYER_JUMP * time.delta_seconds();
//             }
//         }
//     }
// }

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        GameSettings::load_config(app);
        app.insert_resource(CameraZoom { zoom: 0.5 });
        app.insert_resource(LevelFile {
            path: "assets/levels/level_0.xml".into()
        });
        app.insert_resource(PlayerJumpable { can_jump: false });
        app.add_event::<PlayerJumpEvent>();
        app.add_systems(Startup, setup_game);
        app.add_systems(Startup, load_objects);
        app.add_systems(Update, move_player_left_right);
        app.add_systems(Update, keymap);
        app.add_systems(Update, (handle_collision, handle_player_jump).chain());
    }
}
