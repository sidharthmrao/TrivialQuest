use crate::defines::GRAVITY;
use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub const ZERO: Velocity = Velocity(Vec2::ZERO);
}

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

pub struct PhysicsPlugin;

/// Applies a constant acceleration to all `Gravity` objects.
fn apply_gravity(
    mut query: Query<(&mut Transform, &mut Velocity), With<Gravity>>,
    time: Res<Time>
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 += GRAVITY * time.delta_seconds();
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}
