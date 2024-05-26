//! Movement systems.

use super::{
    collider::{ColliderBehavior, Hitbox},
    Fixed, Movable, PhysicsObject
};
use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*
};

/// A `CollisionEvent { movable, fixed, timestamp }` is produced when a solid
/// `movable` collides with a solid `fixed`.
#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub movable: PhysicsObject,
    pub fixed: PhysicsObject,
    pub timestamp: f32
}

/// Collides solid `Movable` objects (with position and velocity) with
/// `Fixed` objects, preventing movement of solid movables through solid fixed
/// objects (see [`ColliderBehavior::Solid`]).
pub fn move_and_collide(
    mut movable_query: Query<
        (&PhysicsObject, &mut Movable, &Hitbox, &ColliderBehavior),
        Without<Fixed>
    >,
    fixed_query: Query<
        (&PhysicsObject, &Transform, &Hitbox, &ColliderBehavior),
        (With<Fixed>, Without<Movable>)
    >,
    time: Res<Time>, mut collision_events: EventWriter<CollisionEvent>
) {
    for (movable_object, mut mover_movable, mover_hitbox, mover_behavior) in
        movable_query.iter_mut()
    {
        let delta_pos = mover_movable.vel().clone() * time.delta_seconds();
        *mover_movable.pos_mut() += delta_pos;

        if *mover_behavior == ColliderBehavior::None {
            continue;
        }

        let mut new_pos = mover_movable.pos().clone();

        for (fixed_object, fixed_transform, fixed_hitbox, fixed_behavior) in
            fixed_query.iter()
        {
            match (mover_hitbox, fixed_hitbox) {
                (Hitbox::AABB(mover_aabb), Hitbox::AABB(fixed_aabb)) => {
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
                        collision_events.send(CollisionEvent {
                            movable: movable_object.clone(),
                            fixed: fixed_object.clone(),
                            timestamp: time.elapsed_seconds()
                        });

                        if *fixed_behavior == ColliderBehavior::None {
                            continue;
                        }
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
                                    fixed_aabb.center().x
                                        - fixed_aabb.half_size().x
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

/// Realizes the now-satisfiable requested movement of movables.
pub fn carry_out_movement(mut query: Query<(&mut Transform, &Movable)>) {
    for (mut transform, movable) in query.iter_mut() {
        transform.translation = movable.pos().extend(0.0);
    }
}
