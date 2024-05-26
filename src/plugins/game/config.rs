use crate::plugins::render::Asset;
use bevy::{math::Vec2, prelude::Color};

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
                Vec2::new(24.0, 24.0)
            ),
            SpritePaths::ENEMY => Asset::new(
                "textures/environment/Tiles/Characters/tile_0004.png",
                Vec2::new(24.0, 24.0)
            ),
            SpritePaths::GRASS => Asset::new(
                "textures/environment/Tiles/tile_0000.png",
                Vec2::new(18.0, 18.0)
            )
        }
    }
}
