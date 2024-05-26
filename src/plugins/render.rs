//! Author: Sidharth Rao
//!
//! The render plugin handles game rendering, texturing, and cameras.
//!
//! It creates a [`MainCamera`] entity and follows an entity with the
//! [`CameraFollow`] component (usually the player). If no entity has the
//! [`CameraFollow`] component, the camera will remain stationary. There can be
//! at most one entity with the [`MainCamera`] component.
//!
//! The plugin also loads sprites by querying for entities with the
//! [`AssetPath`] component and the [`Transform`] component. It then replaces
//! the [`Transform`] component with a [`SpriteBundle`] component containing the
//! texture loaded from the [`AssetPath`] component and the original
//! [`Transform`] component.
//!
//! Schedule:
//!     - Startup: Sets up the main camera.
//!     - Update: Loads sprites. Only processes entities with the [`AssetPath`]
//!       component that has changed. This means that users can change the
//!       texture of an entity by changing the [`AssetPath`] component.
//!     - PostUpdate: Follows the entity (or no entity) with the
//!       [`CameraFollow`] component.

use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use crate::plugins::game::shared::SpritePaths;

/// Handles game rendering, texturing, and cameras.
pub struct RenderPlugin;

/// Main camera.
#[derive(Component)]
pub struct MainCamera;

/// If exactly one entity has this component, the camera will follow it.
#[derive(Component)]
pub struct CameraFollow;

/// Path to the asset file.
#[derive(Component)]
pub struct AssetPath(pub String);

/// Bounding box of a sprite.
#[derive(Component)]
pub struct Size(pub Aabb2d);

// Initializes the main camera as a 2D camera.
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// Follows an entity with the CameraFollow component.
fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    query: Query<&Transform, (With<CameraFollow>, Without<MainCamera>)>
) {
    // Ensure there is exactly one entity with the CameraFollow component.
    if camera_query.iter().count() != 1 || query.iter().count() != 1 {
        return;
    }

    // Move the camera to the entity with the CameraFollow component.
    let mut camera_transform = camera_query.single_mut();
    let transform = query.single();
    camera_transform.translation = transform.translation;
}

// Loads sprites from AssetPath components.
fn make_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Only process entities with an AssetPath component that has changed.
    objects: Query<(Entity, &SpritePaths, &mut Transform), Changed<SpritePaths>>
) {
    for (entity, sprite, transform) in objects.iter() {
        // Remove the old Transform component and insert a SpriteBundle
        // component.
        commands.entity(entity).insert(SpriteBundle {
            texture: asset_server.load(sprite.image_path().clone()),
            transform: *transform,
            ..default()
        });
    }
}

// fn update_bounds(
//     mut commands: Commands,
//     asset_server: Res<Assets<Image>>,
//     // Only process entities with an AssetPath component that has changed.
//     objects: Query<
//         (Entity, &mut Transform, &Handle<Image>),
//         (Changed<Handle<Image>>, With<Sprite>)
//     >
// ) {
//     for (entity, transform, handle) in objects.iter() {
//         let image_dimensions = asset_server.get(handle);
//         match image_dimensions {
//             Some(image) => {
//                 let image_dimensions = image.size_f32();
//                 let scaled_image_dimension = image_dimensions * transform.scale.truncate();
//
//                 let center = vec2(0., 0.);
//                 let half_size = scaled_image_dimension / 2.0;
//                 let bounds = Aabb2d::new(center, half_size);
//
//                 println!("{:?}", bounds);
//
//                 commands.entity(entity).remove::<RenderBounds>();
//                 commands.entity(entity).insert(RenderBounds(bounds));
//             }
//             None => {
//                 return
//             }
//         }
//     }
// }

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, make_sprites);
        // app.add_systems(Update, (make_sprites, update_bounds).chain());
        app.add_systems(PostUpdate, camera_follow_player);
    }
}
