use godot::prelude::*;

use crate::inventory::item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct InventorySlot {
    base: Base<Resource>,
    #[export]
    pub item: Option<Gd<InventoryItem>>,
    #[export]
    pub quantity: u32,
}
