use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node2D)]
pub struct Tool {
    base: Base<Node2D>,
}
