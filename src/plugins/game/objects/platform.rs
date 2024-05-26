use crate::{
    id::Gen,
    plugins::{
        game::config::SpritePaths,
        physics::{
            collider::{Collider, ColliderBehavior, Hitbox},
            FixedPhysicsObject, PhysicsObject
        },
        render::{Asset}
    }
};
use bevy::prelude::*;
use std::fmt::Display;
use crate::plugins::game::entities::enemy::Taxonomy;

pub const PLATFORM_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Platform;

impl Platform {
    pub fn spawn(
        commands: &mut Commands, location: Vec2, platform_type: PlatformType,
        scale: Vec2
    ) {
        println!("Spawning platform at ({}, {}) with scale ({}, {})", location.x, location.y, scale.x, scale.y);
        println!("Hitbox size: ({}, {})", platform_type.asset().size.x / 2.0 * scale.x, platform_type.asset().size.y / 2.0 * scale.y);

        commands.spawn((
            platform_type.asset(),
            Transform::from_xyz(location.x, location.y, 0.0).with_scale(
                Vec3::new(scale.x, scale.y, 1.0)),
            GlobalTransform::IDENTITY,
            Platform,
            FixedPhysicsObject::from(
                PhysicsObject::UniqueNameAndNumber(
                    "platform".into(),
                    Gen::next("platforms")
                ),
                Collider::from(
                    Hitbox::from_size(
                        platform_type.asset().size.x / 2.0 * scale.x,
                        platform_type.asset().size.y / 2.0 * scale.y
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
    pub(crate) fn from_string(str: &str) -> Option<PlatformType> {
        match str {
            "Grass" => Some(PlatformType::Grass),
            _ => None
        }
    }
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
