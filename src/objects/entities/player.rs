use bevy::prelude::{Commands, Component};

use super::shared_components::*;

#[derive(Component)]
pub struct Player {
    pub name: Name,
    pub health: Health,
    pub strength: Strength,
}

pub fn spawn_player(mut commands: Commands, player_name: String) {
    commands.spawn(Player {
        name: Name(player_name),
        health: Health(100),
        strength: Strength(10),
    });
}
