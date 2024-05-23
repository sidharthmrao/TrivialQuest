use godot::prelude::*;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
pub struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>
}