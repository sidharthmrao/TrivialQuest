use crate::plugins::physics::{Gravity, Velocity};
use bevy::prelude::{Commands, Component, GlobalTransform, Transform};

use crate::sprites::shared::*;

#[derive(Component)]
pub struct Enemy(pub Taxonomy);

pub fn spawn_enemy(
    mut commands: Commands, enemy_type: Taxonomy, name: Option<String>,
    location: Transform, init_vel: Velocity
) {
    commands.spawn((
        Name(name.unwrap_or("Enemy".to_string())),
        Health(enemy_type.health()),
        Strength(enemy_type.strength()),
        AssetPath(enemy_type.image_path()),
        location,
        init_vel,
        GlobalTransform::IDENTITY,
        Gravity,
        Enemy(enemy_type)
    ));
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
    pub fn to_string(&self) -> String {
        match self {
            Taxonomy::Human => "Enemy Human",
            Taxonomy::Dwarf => "Enemy Dwarf",
            Taxonomy::Elf => "Enemy Elf",
            Taxonomy::NontrivialSolution => "Nontrivial Solution"
        }
        .into()
    }

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
