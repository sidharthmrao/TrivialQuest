use std::fmt::Display;
use bevy::math::bounding::Aabb2d;

use crate::plugins::{
    game::{config::SpritePaths, Health, Name, Strength},
    physics::{Gravity, Movable},
    render::{Asset}
};
use bevy::prelude::*;
use crate::plugins::physics::Collider;
use crate::plugins::render::Scale;

#[derive(Component)]
pub struct Enemy(pub Taxonomy);

impl Enemy {
    pub fn spawn(
        commands: &mut Commands,
        enemy_type: Taxonomy,
        name: Option<String>,
        location: Vec2,
        scale: Vec2,
        velocity: Vec2
    ) {
        commands.spawn((
            Name(name.unwrap_or("Enemy".to_string())),
            Health(enemy_type.health()),
            Strength(enemy_type.strength()),
            enemy_type.asset(),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Gravity,
            Scale(scale),
            Movable::from(location, velocity),
            Collider::AABB(Aabb2d::new(
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 {
                    x: enemy_type.asset().size.x / 2.0,
                    y: enemy_type.asset().size.y / 2.0
                }
            ))
        ));
    }
}

// Describes the enemy type. This is used to determine the enemy's health and
// strength.
pub enum Taxonomy {
    Human,
    Dwarf,
    Elf,
    NontrivialSolution
}

impl Taxonomy {
    pub fn health(&self) -> u32 {
        match self {
            Taxonomy::Human => 100,
            Taxonomy::Dwarf => 150,
            Taxonomy::Elf => 50,
            Taxonomy::NontrivialSolution => 1000
        }
    }

    pub fn strength(&self) -> u32 {
        match self {
            Taxonomy::Human => 10,
            Taxonomy::Dwarf => 20,
            Taxonomy::Elf => 5,
            Taxonomy::NontrivialSolution => 100
        }
    }

    pub fn asset(&self) -> Asset {
        match self {
            Taxonomy::Human => SpritePaths::ENEMY.asset(),
            Taxonomy::Dwarf => SpritePaths::ENEMY.asset(),
            Taxonomy::Elf => SpritePaths::ENEMY.asset(),
            Taxonomy::NontrivialSolution => SpritePaths::ENEMY.asset(),
        }
    }
}

impl Display for Taxonomy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Taxonomy::Human => "Enemy Human",
            Taxonomy::Dwarf => "Enemy Dwarf",
            Taxonomy::Elf => "Enemy Elf",
            Taxonomy::NontrivialSolution => "Nontrivial Solution"
        }
        .fmt(f)
    }
}
