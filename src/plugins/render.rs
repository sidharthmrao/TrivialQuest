use crate::sprites::shared::AssetPath;
use bevy::prelude::*;

// Handles game rendering, texturing, and cameras.
pub struct RenderPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// Follows an entity with the CameraFollow component.
pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    query: Query<&Transform, (With<CameraFollow>, Without<MainCamera>)>
) {
    if camera_query.iter().count() != 1 || query.iter().count() != 1 {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let transform = query.single();
    camera_transform.translation = transform.translation;
}

fn make_sprites(
    mut commands: Commands, asset_server: Res<AssetServer>,
    objects: Query<(Entity, &AssetPath, &mut Transform), Changed<AssetPath>>
) {
    for (entity, asset_path, transform) in objects.iter() {
        println!("Loading sprite: {:?}", asset_path.0.clone());
        println!("Transform: {:?}", transform.clone());
        commands.entity(entity).remove::<Transform>();
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
        app.add_systems(Update, camera_follow_player);
    }
}
