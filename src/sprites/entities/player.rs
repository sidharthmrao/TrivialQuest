use crate::{
    plugins::physics::{Gravity, Velocity},
    sprites::shared::{AssetPath, Health, Name, SpritePaths, Strength}
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    name: String,
    health: u32,
    strength: u32,
    location: Transform,
    init_vel: Velocity
) {
    commands.spawn((
        Player,
        Name(name),
        Health(health),
        Strength(strength),
        AssetPath(SpritePaths::PLAYER.to_string()),
        location,
        GlobalTransform::IDENTITY,
        init_vel,
        Gravity,
    ));
}
