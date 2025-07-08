use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node2D)]
pub struct Drill {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Drill {
    fn ready(&mut self) {
        self.base_mut().print_tree_pretty();
    }

    fn physics_process(&mut self, delta: f64) {
        let mouse_pos = self.base().get_global_mouse_position();
        self.base_mut().look_at(mouse_pos);
    }
}

pub trait Tool {}

#[godot_dyn]
impl Tool for Drill {}
