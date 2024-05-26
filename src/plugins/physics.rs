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

pub mod collider;
pub mod gravity;
pub mod movement;

use bevy::prelude::*;
use gravity::apply_gravity;
use movement::{carry_out_movement, move_and_collide};

use self::{collider::Collider, gravity::Gravity, movement::CollisionEvent};

/// The physics engine uses this component to uniquely identify physics objects.
#[derive(Component, PartialEq, Eq, Clone, Debug)]
pub enum PhysicsObject {
    UniqueName(String),
    UniqueNumber(i32),
    UniqueNameAndNumber(String, i32)
}

impl PhysicsObject {
    pub fn matches(&self, query: String) -> bool {
        match &self {
            Self::UniqueName(name) | Self::UniqueNameAndNumber(name, _) => {
                name.contains(&query)
            }
            Self::UniqueNumber(_) => false
        }
    }
}

/// A movable physics object (known as a "movable"). For a sprite to have its
/// movement managed by the physics engine (i.e., to support collisions and
/// gravity), you must do two things:
///
/// 1. Add a `Transform` component.
/// 2. Add a [`Movable`] component via [`MovablePhysicsObject`].
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
    #[allow(dead_code)]
    private_constructor: (),

    /// The *requested* position of the object.
    position: Vec2,

    /// The *requested* velocity of the object.
    velocity: Vec2
}

impl Movable {
    /// The current *requested* position.
    pub fn pos(&self) -> &Vec2 {
        &self.position
    }

    /// See [`Movable::pos`].
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    /// /// The current *requested* velocity.
    pub fn vel(&self) -> &Vec2 {
        &self.velocity
    }

    /// See [`Movable::vel`].
    pub fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }
}

/// A physics object with movement and collisions. Read the discussion at
/// [`Movable`] for very important semantics.
#[derive(Bundle)]
pub struct MovablePhysicsObject {
    object: PhysicsObject,
    movable: Movable,
    gravity: Gravity,
    collider: Collider
}

impl MovablePhysicsObject {
    /// Constructs a new movable object.
    pub fn from(
        object: PhysicsObject, position: Vec2, velocity: Vec2,
        has_gravity: bool, collider: Collider
    ) -> Self {
        Self {
            object,
            movable: Movable {
                private_constructor: (),
                position,
                velocity
            },
            gravity: if has_gravity {
                Gravity::Enabled
            } else {
                Gravity::Disabled
            },
            collider
        }
    }
}

/// A fixed physics object.
#[derive(Component)]
pub struct Fixed {
    #[allow(dead_code)]
    private_constructor: ()
}

/// A physics object fixed in place and responding to collisions. Read the
/// discussion at [`Fixed`] for details.
#[derive(Bundle)]
pub struct FixedPhysicsObject {
    object: PhysicsObject,
    fixed: Fixed,
    collider: Collider
}

impl FixedPhysicsObject {
    /// Constructs a new fixed object,
    pub fn from(object: PhysicsObject, collider: Collider) -> Self {
        Self {
            object,
            fixed: Fixed {
                private_constructor: ()
            },
            collider
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_gravity, move_and_collide, carry_out_movement).chain()
        );
        app.add_event::<CollisionEvent>();
    }
}
