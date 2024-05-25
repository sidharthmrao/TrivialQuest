//! Author: Ethan
//!
//! Physics are an essential part of any video game. This plugin handles basic
//! mechanics, including movement, gravity, and collisions.
//!
//! There are two main components: [`Movable`] and [`Fixed`], both of which are
//! managed by the physics engine. An object cannot be both [`Movable`]
//! and [`Fixed`]. For example, players and enemies should be [`Movable`], while
//! platforms and walls should be [`Fixed`].
//!
//! When managing the position of a [`Movable`] object, please use
//! [`Movable::pos`] rather than setting the transform directly (except
//! initally). The physics engine will automatically handle updating the
//! transform.
//!
//! Additional components control behavior and interaction with other physics
//! objects. They include:
//! - [`Gravity`] makes a [`Movable`] affected by gravity.
//! - [`Collider`] makes a [`Movable`] collide with [`Fixed`] objects that are
//!   also [`Collider`].
//!
//! During Bevy's `Update`, the physics engine will apply gravity, check for
//! collisions, and move all [`Movable`]s by updating their transforms.
//!
//! A good thing to do now would be to read the instructions for how to use
//! [`Movable`].

use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*
};

/// The acceleration to due gravity.
pub const GRAVITY: Vec2 = Vec2::new(0.0, -200.0);

/// Use this component on a [`Movable`] to enable gravity.
#[derive(Component)]
pub struct Gravity;

/// A movable physics object. For a sprite to have its movement managed by the
/// physics engine (i.e., to support collisions and gravity), you must do two
/// things:
///
/// 1. Add a `Transform` component.
/// 2. Add a [`Movable`] component.
///
/// When spawning the sprite, initialize the `Transform` and [`Movable`]
/// components to the initial position of the sprite, and the [`Movable`]
/// component to the initial velocity. Afterward, use **ONLY** the [`Movable`]
/// component to manage position.
///
/// Movables work by position and velocity requests. Use [`Movable::pos_mut`]
/// and [`Movable::vel_mut`] to modify the requested position and velocity. (It
/// is recommended to only use delta offsets when modifying the position
/// request.) The physics engine will internally calculate the best satisfiable
/// movement for the requested position and velocity.
#[derive(Component, Clone, Copy)]
pub struct Movable {
    /// The *requested* position of the object.
    position: Vec2,

    /// The *requested* velocity of the object.
    velocity: Vec2
}

impl Movable {
    pub fn new() -> Movable {
        Movable {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO
        }
    }

    pub fn from(position: Vec2, velocity: Vec2) -> Movable {
        Movable { position, velocity }
    }

    pub fn pos(&self) -> &Vec2 {
        &self.position
    }

    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    pub fn vel(&self) -> &Vec2 {
        &self.velocity
    }

    pub fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }
}

#[derive(Component)]
pub struct Fixed;

#[derive(Component)]
pub enum Collider {
    AABB(Aabb2d)
}

#[derive(Event, Default)]
struct CollisionEvent;

pub struct PhysicsPlugin;

/// Applies a constant acceleration to all `Movable` `Gravity` objects.
fn apply_gravity(
    mut query: Query<&mut Movable, With<Gravity>>, time: Res<Time>
) {
    for mut movable in query.iter_mut() {
        *movable.vel_mut() += GRAVITY * time.delta_seconds();
    }
}

/// Collides `Movable` `Collider` objects (with position and velocity) with
/// `Fixed` `Collider` objects.
fn move_and_collide(
    mut movable_query: Query<(&mut Movable, &Collider), Without<Fixed>>,
    fixed_query: Query<
        (&Transform, &Collider),
        (With<Fixed>, Without<Movable>)
    >,
    time: Res<Time>
) {
    for (mut mover_movable, mover_collider) in movable_query.iter_mut() {
        let delta_pos = mover_movable.vel().clone() * time.delta_seconds();
        *mover_movable.pos_mut() += delta_pos;
        let mut new_pos = mover_movable.pos().clone();

        for (fixed_transform, fixed_collider) in fixed_query.iter() {
            match (mover_collider, fixed_collider) {
                (Collider::AABB(mover_aabb), Collider::AABB(fixed_aabb)) => {
                    let mover_aabb = Aabb2d::new(
                        mover_aabb.center() + mover_movable.position,
                        mover_aabb.half_size()
                    );
                    let fixed_aabb = Aabb2d::new(
                        fixed_aabb.center()
                            + fixed_transform.translation.truncate(),
                        fixed_aabb.half_size()
                    );
                    if mover_aabb.intersects(&fixed_aabb) {
                        let dx = mover_movable.position.x
                            - fixed_transform.translation.x;
                        let dy = mover_movable.position.y
                            - fixed_transform.translation.y;

                        let overlap_x = mover_aabb.half_size().x
                            + fixed_aabb.half_size().x
                            - dx.abs();
                        let overlap_y = mover_aabb.half_size().y
                            + fixed_aabb.half_size().y
                            - dy.abs();

                        if overlap_x > overlap_y {
                            mover_movable.vel_mut().y = 0.0;
                            if dy > 0.0 {
                                // Top side of fixed_aabb is intersecting
                                new_pos.y = f32::max(
                                    new_pos.y,
                                    fixed_aabb.center().y
                                        + fixed_aabb.half_size().y
                                        + mover_aabb.half_size().y
                                );
                            } else {
                                // Bottom side of fixed_aabb is intersecting
                                new_pos.y = f32::min(
                                    new_pos.y,
                                    fixed_aabb.center().y
                                        - fixed_aabb.half_size().y
                                        - mover_aabb.half_size().y
                                );
                            }
                        } else {
                            mover_movable.vel_mut().x = 0.0;
                            if dx > 0.0 {
                                // Right side of fixed_aabb is intersecting
                                new_pos.x = f32::max(
                                    new_pos.x,
                                    fixed_aabb.center().x
                                        + fixed_aabb.half_size().x
                                        + mover_aabb.half_size().x
                                );
                            } else {
                                // Left side of fixed_aabb is intersecting
                                new_pos.x = f32::min(
                                    new_pos.x,
                                    mover_aabb.center().x
                                        - mover_aabb.half_size().x
                                        - mover_aabb.half_size().x
                                );
                            }
                        }
                    }
                }
            }
        }

        mover_movable.pos_mut().x = new_pos.x;
        mover_movable.pos_mut().y = new_pos.y;
    }
}

fn carry_out_movement(mut query: Query<(&mut Transform, &Movable)>) {
    for (mut transform, movable) in query.iter_mut() {
        transform.translation = movable.pos().extend(0.0);
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_gravity, move_and_collide, carry_out_movement).chain()
        );
    }
}
