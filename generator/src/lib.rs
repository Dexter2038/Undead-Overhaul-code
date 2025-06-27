use godot::prelude::*;

use crate::player::Player;

mod inventory;
mod pickable;
mod player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Generator {
    base: Base<Node>,
}

#[godot_api]
impl INode for Generator {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Started!");

        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("Ready!");
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for Generator {}
unsafe impl ExtensionLibrary for Player {}
