use crate::sprites::shared::{AssetPath, SpritePaths};
use bevy::prelude::*;

// Handles game rendering, texturing, and cameras.
pub struct RenderPlugin;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn make_sprites(
    mut commands: Commands, asset_server: Res<AssetServer>,
    objects: Query<(Entity, &AssetPath, &mut Transform), Changed<AssetPath>>
) {
    for (entity, asset_path, transform) in objects.iter() {
        println!("Loading sprite: {:?}", asset_path.0.clone());
        println!("Transform: {:?}", transform.clone());
        commands.entity(entity).insert(SpriteBundle {
            texture: asset_server.load(asset_path.0.clone()),
            transform: *transform,
            ..default()
        });
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, make_sprites);
    }
}
