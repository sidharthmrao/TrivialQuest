use bevy::math::{Vec2, vec2};
use bevy::prelude::{Color, Component};

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PLAYER_HORIZONTAL_MOVEMENT_SPEED: f32 = 200.0;

#[derive(Debug, Clone, Copy, Component)]
pub enum SpritePaths {
    PLAYER,
    ENEMY,
    GRASS
}

impl SpritePaths {
    pub fn image_path(&self) -> String {
        match self {
            SpritePaths::PLAYER => "textures/environment/Tiles/Characters/tile_0000.png",
            SpritePaths::ENEMY => "textures/environment/Tiles/Characters/tile_0004.png",
            SpritePaths::GRASS => "textures/environment/Tiles/tile_0000.png"
        }.into()
    }

    pub fn size(&self) -> Vec2 {
        match self {
            SpritePaths::PLAYER => vec2(24.0, 24.0),
            SpritePaths::ENEMY => vec2(24.0, 24.0),
            SpritePaths::GRASS => vec2(18.0, 18.0)
        }
    }
}
