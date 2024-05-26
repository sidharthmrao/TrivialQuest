use bevy::math::{Vec2, vec2};
use bevy::prelude::{Color, Component};
use crate::plugins::render::Asset;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PLAYER_HORIZONTAL_MOVEMENT_SPEED: f32 = 200.0;

#[derive(Debug, Clone, Copy)]
pub enum SpritePaths {
    PLAYER,
    ENEMY,
    GRASS
}

impl SpritePaths {
    pub fn asset(&self) -> Asset {
        match self {
            SpritePaths::PLAYER => Asset::new(
                "textures/environment/Tiles/Characters/tile_0000.png",
                vec2(24.0, 24.0)),
            SpritePaths::ENEMY => Asset::new(
                "textures/environment/Tiles/Characters/tile_0004.png",
                vec2(24.0, 24.0)),
            SpritePaths::GRASS => Asset::new(
                "textures/environment/Tiles/tile_0000.png",
                vec2(18.0, 18.0))
        }
    }
}
