use bevy::prelude::{Commands, Component};

use super::shared_components::*;

#[derive(Component)]
pub struct Player {
    pub name: Name,
    pub health: Health,
    pub strength: Strength,
    pub asset_path: AssetPath
}

impl Player {
    pub fn new(name: String, health: u32, strength: u32, asset_path: String) -> Self {
        Self {
            name: Name(name),
            health: Health(health),
            strength: Strength(strength),
            asset_path: AssetPath(asset_path)
        }
    }
}
