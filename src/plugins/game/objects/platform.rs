use std::fmt::Display;
use crate::plugins::physics::{Collider, Fixed};
use bevy::{math::bounding::Aabb2d, prelude::*};
use crate::plugins::game::shared::SpritePaths;
use crate::plugins::render::AssetPath;

pub const PLATFORM_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Platform;

impl Platform {
    pub fn spawn(
        commands: &mut Commands,
        location: Vec2,
        platform_type: PlatformType
    ) {
        commands.spawn((
            platform_type.asset(),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Platform,
            Fixed,
            Collider::AABB(Aabb2d::new(
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 {
                    x: platform_type.asset().size().x / 2.0,
                    y: platform_type.asset().size().y / 2.0
                }
            ))
        ));
    }
}

// Describes the platform type.
pub enum PlatformType {
    Grass,
}

impl PlatformType {
    pub fn asset(&self) -> SpritePaths {
        match self {
            PlatformType::Grass => SpritePaths::GRASS,
        }
    }
}

impl Display for PlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformType::Grass => write!(f, "Grass"),
        }
    }
}