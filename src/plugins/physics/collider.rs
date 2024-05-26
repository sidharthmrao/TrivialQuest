use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*
};

#[derive(Component)]
pub enum Hitbox {
    /// The bounding box is relative to the current transform of the player, so
    /// typically the box passed here will be centered at `Vec2::ZERO`.
    AABB(Aabb2d)
}

impl Hitbox {
    pub fn from_size(width: f32, height: f32) -> Self {
        Self::AABB(Aabb2d::new(Vec2::ZERO, Vec2::new(width, height)))
    }

    pub fn set_size(&mut self, size: Vec2) {
        match self {
            Hitbox::AABB(aabb) => {
                aabb.max = aabb.center() + size / 2.;
                aabb.min = aabb.center() - size / 2.;
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum ColliderBehavior {
    /// This object does not cause an infeasible location for
    /// [`ColliderBehavior::Solid`] objects.
    None,

    /// This object causes an infeasible location for [`ColliderType::Solid`]
    /// objects.
    Solid
}

#[derive(Bundle)]
pub struct Collider {
    pub hitbox: Hitbox,
    pub behavior: ColliderBehavior
}

impl Collider {
    pub fn from(hitbox: Hitbox, behavior: ColliderBehavior) -> Collider {
        Collider { hitbox, behavior }
    }
}
