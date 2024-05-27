use super::Movable;
use bevy::prelude::*;

/// The settings for the physics plugin.
#[derive(Resource)]
pub struct PhysicsSettings {
    /// The acceleration due to gravity.
    pub(crate) gravity: Vec2
}

/// Use this component on a [`Movable`] to enable gravity.
#[derive(Component, PartialEq, Eq)]
pub enum Gravity {
    Enabled,
    Disabled
}

/// Applies a constant acceleration to all `Movable` `Gravity` objects.
pub fn apply_gravity(
    mut query: Query<(&mut Movable, &Gravity)>, time: Res<Time>,
    settings: Res<PhysicsSettings>
) {
    for (mut movable, gravity) in query.iter_mut() {
        if *gravity == Gravity::Enabled {
            *movable.vel_mut() += settings.gravity * time.delta_seconds();
        }
    }
}
