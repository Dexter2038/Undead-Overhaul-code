use godot::prelude::*;

use crate::inventory::{item::InventoryItem, slot::InventorySlot};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Inventory {
    base: Base<Resource>,

    #[export]
    #[init(val = 25)]
    pub size: u32,

    #[export]
    #[init(val = 3)]
    pub hotbar_size: u32,

    #[export]
    slots: Array<Gd<InventorySlot>>,
}

impl Inventory {
    #[allow(dead_code)]
    pub fn expand(&mut self, size: u32) -> Result<(), ()> {
        if size <= self.size {
            return Err(());
        }
        self.size = size;
        let new_size = (self.hotbar_size + self.size) as usize;
        let new_slot = InventorySlot::new_gd();
        self.slots.resize(new_size, &new_slot);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn remove_from_slot(&mut self, slot: u32 /*TODO: , quantity: u32*/) -> Result<(), ()> {
        let Some(mut slot) = self.slots.get(slot as usize) else {
            return Err(());
        };

        let mut slot = slot.bind_mut();
        slot.item = None;
        slot.quantity = 0;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn add_item(&mut self, item: &Gd<InventoryItem>, mut quantity: u32) -> Result<(), u32> {
        let item_obj = item.bind();
        let max_stack = item_obj.get_max_stack();

        // First, try to fill existing stacks of the same item
        for mut slot in self.slots.iter_shared() {
            if quantity == 0 {
                break;
            }

            let mut slot = slot.bind_mut();

            if let Some(existing_item) = &slot.item {
                if existing_item.get_name().to_string() == item_obj.get_name().to_string()
                    && slot.quantity < max_stack
                {
                    let available_space = max_stack - slot.quantity;
                    let to_add = available_space.min(quantity);
                    slot.quantity += to_add;
                    quantity -= to_add;
                }
            }
        }

        // Then, try to use empty slots for remaining quantity
        for mut slot in self.slots.iter_shared() {
            if quantity == 0 {
                break;
            }

            let mut slot = slot.bind_mut();

            if slot.item.is_none() {
                let to_add = max_stack.min(quantity);
                slot.item = Some(item.clone());
                slot.quantity = to_add;
                quantity -= to_add;
            }
        }

        if quantity == 0 { Ok(()) } else { Err(quantity) }
    }

    #[allow(dead_code)]
    pub fn move_item(&mut self, from: u32, to: u32) -> Result<(), ()> {
        let (Some(mut from_slot), Some(mut to_slot)) =
            (self.slots.get(from as usize), self.slots.get(to as usize))
        else {
            return Err(());
        };

        let mut from = from_slot.bind_mut();
        let mut to = to_slot.bind_mut();

        std::mem::swap(&mut from.item, &mut to.item);
        std::mem::swap(&mut from.quantity, &mut to.quantity);

        Ok(())
    }
}
