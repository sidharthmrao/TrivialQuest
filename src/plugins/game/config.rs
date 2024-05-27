use crate::plugins::render::Asset;
use bevy::{math::Vec2, prelude::Color};
use bevy::prelude::{App, ClearColor, Resource};
use crate::plugins::physics::gravity::PhysicsSettings;

#[derive(Debug, Resource)]
pub struct GameSettings {
    pub player_jump_height: f32,
    pub jump_time: f32,
    pub player_jump_vel: f32,
    pub player_horizontal_movement_speed: f32
}

impl GameSettings {
    pub fn load_config(app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));

        let player_jump_height: f32 = 24.0; // Pixels
        let jump_time: f32 = 0.16; // Seconds
        let gravity: f32 = -player_jump_height / 2.0 / jump_time / jump_time; // Pixels per
        // second^2
        let player_jump_vel: f32 = (2.0 * player_jump_height * -gravity).sqrt(); // Pixels per
        // second
        let player_horizontal_movement_speed: f32 = 75.0; // Pixels per second

        app.insert_resource(PhysicsSettings {
            gravity: Vec2::new(0.0, gravity)
        });

        app.insert_resource(Self {
            player_jump_height,
            jump_time,
            player_jump_vel,
            player_horizontal_movement_speed
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpritePaths {
    PLAYER,
    ENEMY,
    GRASS,
    SLIME
}

impl SpritePaths {
    pub fn asset(&self) -> Asset {
        match self {
            SpritePaths::PLAYER => Asset::new(
                "textures/environment/Tiles/Characters/tile_0000.png",
                Vec2::new(24.0, 24.0)
            ),
            SpritePaths::ENEMY => Asset::new(
                "textures/environment/Tiles/Characters/tile_0004.png",
                Vec2::new(24.0, 24.0)
            ),
            SpritePaths::GRASS => Asset::new(
                "textures/environment/Tiles/tile_0000.png",
                Vec2::new(18.0, 18.0)
            ),
            SpritePaths::SLIME => Asset::new(
                "textures/environment/Tiles/tile_0018.png",
                Vec2::new(18.0, 18.0)
            )
        }
    }
}
