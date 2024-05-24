use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Strength(pub u32);
