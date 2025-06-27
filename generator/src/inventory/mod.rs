pub mod item;
pub mod slot;

use godot::{classes::Control, prelude::*};

use crate::inventory::{item::InventoryItem, slot::InventorySlot};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Inventory {
    base: Base<Control>,

    size: u32,
    slots: Array<Gd<InventorySlot>>,
    hotbar_size: u32,
}

impl Inventory {
    #[allow(dead_code)]
    pub fn add_item(&mut self, item: &InventoryItem) -> Result<(), ()> {
        let mut available_idx = None;
        for (idx, slot) in self.slots.iter_shared().enumerate() {
            let slot = slot.bind();
            if slot.item.is_none() {
                available_idx = Some(idx);
                break;
            }
            let slot_item = slot.item.as_ref().unwrap();
            if slot_item.get_name() != item.get_name() {
                continue;
            }
            if slot.quantity >= item.get_max_stack() {
                continue;
            }
            available_idx = Some(idx);
            break;
        }
        if available_idx.is_none() {
            return Err(());
        }

        let available_idx = available_idx.unwrap();
        let slot = self.slots.get(available_idx);
        if slot.is_none() {
            return Err(());
        }

        let slot = slot.unwrap();

        Ok(())
    }
}
