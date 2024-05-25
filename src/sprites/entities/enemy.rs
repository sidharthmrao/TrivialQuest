use bevy::prelude::{Commands, Component};

use crate::sprites::shared_components::*;

#[derive(Component)]
pub struct Enemy {
    pub taxonomy: Taxonomy,
    pub strength: Strength,
    pub health: Health,
    pub name: Name,
    pub asset_path: AssetPath
}

pub fn spawn_enemy(
    mut commands: Commands, enemy_type: Taxonomy, name: Option<String>
) {
    commands.spawn(Enemy {
        health: Health(enemy_type.health()),
        strength: Strength(enemy_type.strength()),
        asset_path: AssetPath(enemy_type.image_path()),
        taxonomy: enemy_type,
        name: Name(name.unwrap_or("Enemy".to_string()))
    });
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
            Taxonomy::Human => {
                "textures/sprites/PNG/Side view/robot_blueBody.png"
            }
            Taxonomy::Dwarf => {
                "textures/sprites/PNG/Side view/robot_greenBody.png"
            }
            Taxonomy::Elf => "textures/sprites/PNG/Side view/robot_redBody.png",
            Taxonomy::NontrivialSolution => {
                "textures/sprites/PNG/Side view/robot_yellowBody.png"
            }
        }
        .into()
    }
}
