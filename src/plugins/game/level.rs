use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy::utils::info;

use xml::reader::{EventReader, XmlEvent};
use crate::plugins::game::entities::enemy::Taxonomy;
use crate::plugins::game::objects::platform::PlatformType;

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    strength: u32,
    location: (f32, f32),
    scale: (f32, f32),
    velocity: (f32, f32)
}
struct Enemy {
    name: String,
    taxonomy: Taxonomy,
    location: (f32, f32),
    scale: (f32, f32),
    velocity: (f32, f32)
}
struct Platform {
    location: (f32, f32),
    platform_type: PlatformType,
    scale: (f32, f32)
}

enum ObjectType {
    Player,
    Enemy,
    Platform,
    ObjectArray,
    None
}

impl ObjectType {
    fn from_string(name: &str) -> Self {
        match name {
            "player" => ObjectType::Player,
            "enemy" => ObjectType::Enemy,
            "platform" => ObjectType::Platform,
            "object_array" => ObjectType::ObjectArray,
            _ => ObjectType::None
        }
    }

    fn handle(&self, commands: &mut Commands, attributes: Vec<xml::attribute::OwnedAttribute>) {
        match self {
            ObjectType::Player => {
                let mut player = Player {
                    name: String::new(),
                    health: 0,
                    strength: 0,
                    location: (0.0, 0.0),
                    scale: (0.0, 0.0),
                    velocity: (0.0, 0.0)
                };

                for attr in attributes {
                    println!("{:?}", attr);

                    match attr.name.local_name.as_str() {
                        "name" => player.name = attr.value,
                        "health" => player.health = attr.value.parse().unwrap(),
                        "strength" => player.strength = attr.value.parse().unwrap(),
                        "location" => {
                            let mut split = attr.value.split(", ");

                            player.location = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        "scale" => {
                            let mut split = attr.value.split(", ");
                            player.scale = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        "velocity" => {
                            let mut split = attr.value.split(", ");
                            player.velocity = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        _ => {}
                    }
                }

                crate::plugins::game::entities::player::Player::spawn(
                    commands,
                    player.name,
                    player.health,
                    player.strength,
                    Vec2::new(player.location.0, player.location.1),
                    Vec2::new(player.scale.0, player.scale.1),
                    Vec2::new(player.velocity.0, player.velocity.1)
                );
            },
            ObjectType::Enemy => {
                let mut enemy = Enemy {
                    name: String::new(),
                    taxonomy: Taxonomy::Human,
                    location: (0.0, 0.0),
                    scale: (0.0, 0.0),
                    velocity: (0.0, 0.0)
                };

                for attr in attributes {
                    println!("{:?}", attr);

                    match attr.name.local_name.as_str() {
                        "name" => enemy.name = attr.value,
                        "taxonomy" => enemy.taxonomy = Taxonomy::from_string(attr.value.as_str()).unwrap(),
                        "location" => {
                            let mut split = attr.value.split(", ");

                            enemy.location = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        "scale" => {
                            let mut split = attr.value.split(", ");
                            enemy.scale = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        "velocity" => {
                            let mut split = attr.value.split(", ");
                            enemy.velocity = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        _ => {}
                    }
                }

                crate::plugins::game::entities::enemy::Enemy::spawn(
                    commands,
                    enemy.taxonomy,
                    Some(enemy.name),
                    Vec2::new(enemy.location.0, enemy.location.1),
                    Vec2::new(enemy.scale.0, enemy.scale.1),
                    Vec2::new(enemy.velocity.0, enemy.velocity.1)
                );
            }
            ObjectType::Platform => {
                let mut platform = Platform {
                    location: (0.0, 0.0),
                    platform_type: PlatformType::Grass,
                    scale: (0.0, 0.0)
                };

                for attr in attributes {
                    println!("{:?}", attr);

                    match attr.name.local_name.as_str() {
                        "location" => {
                            let mut split = attr.value.split(", ");

                            platform.location = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        "platform_type" => platform.platform_type = PlatformType::from_string(attr.value.as_str()).unwrap(),
                        "scale" => {
                            let mut split = attr.value.split(", ");
                            platform.scale = (
                                split.next().unwrap().parse().unwrap(),
                                split.next().unwrap().parse().unwrap()
                            );
                        },
                        _ => {}
                    }
                }

                crate::plugins::game::objects::platform::Platform::spawn(
                    commands,
                    Vec2::new(platform.location.0, platform.location.1),
                    platform.platform_type,
                    Vec2::new(platform.scale.0, platform.scale.1)
                );
            }
            ObjectType::ObjectArray => {}
            ObjectType::None => {}
        }
    }
}

pub fn load_objects(mut commands: Commands)  {
    let file = File::open("assets/levels/level_0.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                ObjectType::from_string(name.local_name.as_str()).handle(&mut commands, attributes);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}