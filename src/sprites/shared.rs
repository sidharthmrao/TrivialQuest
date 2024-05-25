use bevy::prelude::*;

pub struct SpritePaths;

impl SpritePaths {
    pub const TILE_SIZE: f32 = 24.0;
    pub const PLAYER: &'static str =
        "textures/environment/Tiles/Characters/tile_0000.png";
    pub const ENEMY: &'static str =
        "textures/environment/Tiles/Characters/tile_0004.png";
    pub const GRASS_BLOCK: &'static str =
        "textures/environment/Tiles/tile_000.png";
}
