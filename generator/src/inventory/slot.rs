use godot::prelude::*;

use crate::inventory::item::InventoryItem;

#[allow(dead_code)]
#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct InventorySlot {
    base: Base<Resource>,
    pub item: Option<Gd<InventoryItem>>,
    pub quantity: u32,
}
