use bevy::prelude::{Commands, Component};

use super::shared_components::*;

pub enum Taxonomy {
    Human,
    Dwarf,
    Elf,
    NontrivialSolution,
}

impl Taxonomy {
    pub fn to_string(&self) -> String {
        match self {
            Taxonomy::Human => "Enemy Human".to_string(),
            Taxonomy::Dwarf => "Enemy Dwarf".to_string(),
            Taxonomy::Elf => "Enemy Elf".to_string(),
            Taxonomy::NontrivialSolution => "Nontrivial Solution".to_string(),
        }
    }

    pub fn health(&self) -> u32 {
        match self {
            Taxonomy::Human => 100,
            Taxonomy::Dwarf => 150,
            Taxonomy::Elf => 50,
            Taxonomy::NontrivialSolution => 1000,
        }
    }

    pub fn strength(&self) -> u32 {
        match self {
            Taxonomy::Human => 10,
            Taxonomy::Dwarf => 20,
            Taxonomy::Elf => 5,
            Taxonomy::NontrivialSolution => 100,
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub taxonomy: Taxonomy,
}

pub fn spawn_enemy(mut commands: Commands, enemy_type: Taxonomy, name: Option<String>) {
    commands.spawn((
        Health(enemy_type.health()),
        Strength(enemy_type.strength()),
        Enemy {
            taxonomy: enemy_type,
        },
        Name(name.unwrap_or("Enemy".to_string())),
    ));
}