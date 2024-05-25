use bevy::prelude::*;
use trivial_quest::objects::entities::player::Player;
use trivial_quest::objects::entities::shared_components::AssetPath;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, (setup, render_assets).chain())
        .run();
}

fn render_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Query<&AssetPath>)
{
    println!("Rendering assets");
    for asset in assets.iter() {
        println!("Rendering asset: {}", asset.0.clone());
        commands.spawn(SpriteBundle {
            texture: asset_server.load(asset.0.clone()),
            ..default()
        });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Player::new("Player".to_string(), 100, 10, "textures/sprites/PNG/Side view/robot_blueBody.png".to_string()));
}
