use crate::sprites::shared_components::AssetPath;
use bevy::prelude::*;

// Handles game rendering, texturing, and cameras.
pub struct RenderPlugin;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// pub fn render_assets(
//     mut commands: Commands, asset_server: Res<AssetServer>,
//     assets: Query<&AssetPath>
// ) {
//     println!("Rendering assets");
//     for asset in assets.iter() {
//         println!("Rendering asset: {}", asset.0.clone());
//         commands.spawn(SpriteBundle {
//             texture: asset_server.load(asset.0.clone()),
//             ..default()
//         });
//     }
// }

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        // app.add_systems(Startup, render_assets);
    }
}
