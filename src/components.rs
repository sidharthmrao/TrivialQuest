use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub const ZERO: Velocity = Velocity(Vec2::ZERO);
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Platform;
