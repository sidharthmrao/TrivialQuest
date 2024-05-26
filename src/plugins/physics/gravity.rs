use super::Movable;
use bevy::prelude::*;

/// The acceleration to due gravity.
pub const GRAVITY: Vec2 = Vec2::new(0.0, -200.0);

/// Use this component on a [`Movable`] to enable gravity.
#[derive(Component, PartialEq, Eq)]
pub enum Gravity {
    Enabled,
    Disabled
}

/// Applies a constant acceleration to all `Movable` `Gravity` objects.
pub fn apply_gravity(
    mut query: Query<(&mut Movable, &Gravity)>, time: Res<Time>
) {
    for (mut movable, gravity) in query.iter_mut() {
        if *gravity == Gravity::Enabled {
            *movable.vel_mut() += GRAVITY * time.delta_seconds();
        }
    }
}
