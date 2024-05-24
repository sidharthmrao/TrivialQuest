use bevy::prelude::{Commands, Component};

use super::shared_components::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, player_name: String) {
    commands.spawn((Player, Name(player_name), Health(100), Strength(10)));
}
