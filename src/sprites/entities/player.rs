use crate::{
    plugins::{
        physics::{Collider, Gravity, Movable},
        render::CameraFollow
    },
    sprites::shared::{AssetPath, Health, Name, SpritePaths, Strength}
};
use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        commands: &mut Commands, name: String, health: u32, strength: u32,
        location: Vec2, velocity: Vec2
    ) {
        commands.spawn((
            Player,
            Name(name),
            Health(health),
            Strength(strength),
            AssetPath(SpritePaths::PLAYER.into()),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Gravity,
            CameraFollow,
            Movable::from(location, velocity),
            Collider::AABB(Aabb2d::new(
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 {
                    x: SpritePaths::TILE_SIZE / 2.0,
                    y: SpritePaths::TILE_SIZE / 2.0
                }
            ))
        ));
    }
}
