use crate::plugins::{
    game::{config::SpritePaths, Health, Name, Strength},
    physics::{
        collider::{Collider, ColliderBehavior, Hitbox},
        MovablePhysicsObject, PhysicsObject
    }
};
use bevy::{prelude::*, utils::info};


#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        commands: &mut Commands, name: String, health: u32, strength: u32,
        location: Vec2, scale: Vec2, velocity: Vec2
    ) {
        info("Spawning player");
        println!(
            "Spawning player at ({}, {}) with scale ({}, {})",
            location.x, location.y, scale.x, scale.y
        );

        commands.spawn((
            Player,
            Name(name),
            Health(health),
            Strength(strength),
            SpritePaths::PLAYER.asset(),
            Transform::from_xyz(location.x, location.y, 0.0)
                .with_scale(Vec3::new(scale.x, scale.y, 1.0)),
            GlobalTransform::IDENTITY,
            // CameraFollow,
            MovablePhysicsObject::from(
                PhysicsObject::UniqueName("player".into()),
                location,
                velocity,
                true,
                Collider::from(
                    Hitbox::from_size(
                        SpritePaths::PLAYER.asset().size.x / 2.0 * scale.x,
                        SpritePaths::PLAYER.asset().size.y / 2.0 * scale.y
                    ),
                    ColliderBehavior::Solid
                )
            )
        ));
    }
}
