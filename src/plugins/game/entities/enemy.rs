use std::fmt::Display;
use bevy::math::bounding::Aabb2d;

use crate::plugins::{
    game::{shared::SpritePaths, Health, Name, Strength},
    physics::{Gravity, Movable},
    render::AssetPath
};
use bevy::prelude::*;
use crate::plugins::game::entities::player::Player;
use crate::plugins::physics::Collider;
use crate::plugins::render::CameraFollow;

#[derive(Component)]
pub struct Enemy(pub Taxonomy);

impl Enemy {
    pub fn spawn(
        commands: &mut Commands,
        enemy_type: Taxonomy,
        name: Option<String>,
        location: Vec2,
        velocity: Vec2
    ) {
        commands.spawn((
            Name(name.unwrap_or("Enemy".to_string())),
            Health(enemy_type.health()),
            Strength(enemy_type.strength()),
            AssetPath(enemy_type.image_path()),
            Transform::from_xyz(location.x, location.y, 0.0),
            GlobalTransform::IDENTITY,
            Gravity,
            Movable::from(location, velocity),
            Collider::AABB(Aabb2d::new(
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 {
                    x: SpritePaths::TILE_SIZE / 2.0,
                    y: SpritePaths::TILE_SIZE / 2.0
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

    pub fn image_path(&self) -> String {
        match self {
            Taxonomy::Human => SpritePaths::ENEMY,
            Taxonomy::Dwarf => SpritePaths::ENEMY,
            Taxonomy::Elf => SpritePaths::ENEMY,
            Taxonomy::NontrivialSolution => SpritePaths::ENEMY
        }
        .into()
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
