use crate::plugins::physics::{Collider, Fixed};
use bevy::{math::bounding::Aabb2d, prelude::*};

pub const PLATFORM_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Platform;

impl Platform {
    pub fn spawn(
        commands: &mut Commands, location: Vec2, width: f32, height: f32
    ) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: PLATFORM_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: location.extend(0.0),
                    scale: Vec3::new(width, height, 1.0),
                    ..default()
                },
                global_transform: GlobalTransform::IDENTITY,
                ..default()
            },
            Platform,
            Fixed,
            Collider::AABB(Aabb2d::new(
                location,
                Vec2::new(width / 2.0, height / 2.0)
            ))
        ));
    }
}
