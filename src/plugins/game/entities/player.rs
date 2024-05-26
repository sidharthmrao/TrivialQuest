use crate::plugins::{
    game::{config::SpritePaths, Health, Name, Strength},
    physics::{Collider, Gravity, Movable},
    render::{CameraFollow}
};
use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        commands: &mut Commands,
        name: String,
        health: u32,
        strength: u32,
        location: Vec2,
        velocity: Vec2
    ) {
        commands.spawn((
            Player,
            Name(name),
            Health(health),
            Strength(strength),
            SpritePaths::PLAYER.asset(),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Gravity,
            CameraFollow,
            Movable::from(location, velocity),
            Collider::AABB(Aabb2d::new(
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 {
                    x: SpritePaths::PLAYER.asset().size.x / 2.0,
                    y: SpritePaths::PLAYER.asset().size.y / 2.0,
                }
            ))
        ));
    }
}
