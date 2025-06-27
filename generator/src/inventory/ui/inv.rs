use godot::{classes::Control, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct InventoryUI {
    base: Base<Control>,
}
