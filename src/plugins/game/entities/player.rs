use crate::plugins::{
    game::{config::SpritePaths, Health, Name, Strength},
    physics::{
        collider::{Collider, ColliderBehavior, Hitbox},
        MovablePhysicsObject, PhysicsObject
    },
    render::Scale
};
use bevy::prelude::*;

pub const PLAYER_JUMP: f32 = 1000.0;

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        commands: &mut Commands, name: String, health: u32, strength: u32,
        location: Vec2, scale: Vec2, velocity: Vec2
    ) {
        commands.spawn((
            Player,
            Name(name),
            Health(health),
            Strength(strength),
            SpritePaths::PLAYER.asset(),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Scale(scale),
            // CameraFollow,
            MovablePhysicsObject::from(
                PhysicsObject::UniqueName("player".into()),
                location,
                velocity,
                true,
                Collider::from(
                    Hitbox::from_size(
                        SpritePaths::PLAYER.asset().size.x / 2.0,
                        SpritePaths::PLAYER.asset().size.y / 2.0
                    ),
                    ColliderBehavior::Solid
                )
            )
        ));
    }
}
