use godot::prelude::*;

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
