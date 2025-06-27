use godot::{classes::RigidBody2D, prelude::*};

use crate::resources::inv_item::InvItem;

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub struct Pickable {
    base: Base<RigidBody2D>,

    #[export]
    #[var]
    item: Option<Gd<InvItem>>,

    #[var]
    pub quantity: u32,
}

#[godot_api]
impl Pickable {
    pub fn item(&self) -> &Gd<InvItem> {
        self.item.as_ref().unwrap()
    }
}
