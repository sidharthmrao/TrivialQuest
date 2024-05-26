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

/// Handles game rendering, texturing, and cameras.
pub struct RenderPlugin;

/// Main camera.
#[derive(Component)]
pub struct MainCamera;

/// If exactly one entity has this component, the camera will follow it.
#[derive(Component)]
pub struct CameraFollow;

/// Contains the path to an asset and its size.
#[derive(Component)]
pub struct Asset {
    pub path: String,
    pub size: Vec2
}

impl Asset {
    /// Creates a new asset with the given path and size.
    pub fn new(path: &str, size: Vec2) -> Self {
        Self { path: path.into(), size }
    }
}

/// Scale of a sprite (default (1, 1)).
#[derive(Component)]
pub struct Scale(pub Vec2);

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
    // Only process entities with an AssetPath or Scale component that has changed.
    objects: Query<(Entity, &Asset, &mut Transform, &Scale), Or<(Changed<Asset>, Changed<Scale>)>>
) {
    for (entity, sprite, transform, scale) in objects.iter() {
        // Remove the old Transform component and insert a SpriteBundle
        // component.
        let transform_ = Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: Vec3::new(scale.0.x, scale.0.y, 1.0),
        };

        commands.entity(entity).insert(SpriteBundle {
            texture: asset_server.load(sprite.path.clone()),
            transform: transform_,
            ..default()
        });
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, make_sprites);
        app.add_systems(PostUpdate, camera_follow_player);
    }
}
