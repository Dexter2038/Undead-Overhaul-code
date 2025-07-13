use godot::prelude::*;

mod drill;
mod inventory;
mod map;
mod pickable;
mod player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Initializer {
    base: Base<Node>,
}

#[godot_api]
impl INode for Initializer {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Init!");

        Self { base }
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for Initializer {}
