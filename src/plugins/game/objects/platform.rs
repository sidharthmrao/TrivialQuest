use crate::{
    id::Gen,
    plugins::{
        game::config::SpritePaths,
        physics::{
            collider::{Collider, ColliderBehavior, Hitbox},
            FixedPhysicsObject, PhysicsObject
        },
        render::{Asset, Scale}
    }
};
use bevy::prelude::*;
use std::fmt::Display;

pub const PLATFORM_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Platform;

impl Platform {
    pub fn spawn(
        commands: &mut Commands, location: Vec2, platform_type: PlatformType,
        scale: Vec2
    ) {
        commands.spawn((
            platform_type.asset(),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Platform,
            Scale(scale),
            FixedPhysicsObject::from(
                PhysicsObject::UniqueNameAndNumber(
                    "platform".into(),
                    Gen::next("platforms")
                ),
                Collider::from(
                    Hitbox::from_size(
                        platform_type.asset().size.x / 2.0,
                        platform_type.asset().size.y / 2.0
                    ),
                    ColliderBehavior::Solid
                )
            )
        ));
    }
}

// Describes the platform type.
pub enum PlatformType {
    Grass
}

impl PlatformType {
    pub fn asset(&self) -> Asset {
        match self {
            PlatformType::Grass => SpritePaths::GRASS.asset()
        }
    }
}

impl Display for PlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformType::Grass => write!(f, "Grass")
        }
    }
}
