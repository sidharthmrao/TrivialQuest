use bevy::prelude::*;

pub mod camera;
use camera::setup_camera;

// Graphics plugin, meant to govern camera, render, etc.
pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}
