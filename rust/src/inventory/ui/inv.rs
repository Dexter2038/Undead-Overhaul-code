use godot::{
    classes::{Control, GridContainer, IControl, NinePatchRect, TextureButton},
    prelude::*,
};

use crate::inventory::{inv::Inventory, ui::slot::InventorySlotUI};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct InventoryUI {
    base: Base<Control>,

    #[export]
    inventory_node: Option<Gd<NinePatchRect>>,

    #[export]
    inv_spawn_point: Option<Gd<GridContainer>>,

    #[export]
    hotbar_spawn_point: Option<Gd<GridContainer>>,

    #[var]
    pub inventory: Option<Gd<Inventory>>,

    #[init(val = Vec::new())]
    slots: Vec<Gd<InventorySlotUI>>,
}

#[godot_api]
impl InventoryUI {
    #[signal]
    pub fn slot_clicked(slot: u32);
}

#[godot_api]
impl IControl for InventoryUI {
    fn ready(&mut self) {
        if self.inv_spawn_point.is_none() {
            godot_warn!("InventoryUI: spawn point is not set");
        }
        if self.hotbar_spawn_point.is_none() {
            godot_warn!("InventoryUI: hotbar spawn point is not set");
        }
        if self.inventory.is_none() {
            godot_warn!("InventoryUI: inventory is not set");
        }
        if self.inventory_node.is_none() {
            godot_warn!("InventoryUI: inventory node is not set");
        }
        self.inventory_node_mut().set_visible(false);
        let slots = self.inventory().bind().get_slots();
        let hotbar_size = self.inventory().bind().hotbar_size as usize;
        {
            let inventory = self.inventory().bind();
            if inventory.size + inventory.hotbar_size != slots.len() as u32 {
                godot_warn!(
                    "InventoryUI: inventory size mismatch: {} != {}",
                    inventory.size + inventory.hotbar_size,
                    slots.len()
                );
            }
        }

        for (idx, i) in slots.iter_shared().enumerate() {
            let scene = load::<PackedScene>("res://scenes/ui/inventory_slot.tscn");
            let Some(scene) = scene.instantiate() else {
                godot_error!("Failed to instantiate inventory slot scene");
                return;
            };
            let slot = i.bind();
            {
                let slot_ui = scene.to_godot().cast::<TextureButton>();
                let idx = idx as u32;
                let self_gd = self.to_gd();
                let self_gd = self_gd.clone();
                let func = move || {
                    // Use the owned Gd (not GdRef)
                    let mut self_gd = self_gd.clone(); // Clone Gd (reference counted)
                    self_gd.bind_mut().signals().slot_clicked().emit(idx);
                };

                slot_ui.signals().pressed().connect(func);
            }
            let mut slot_ui = scene.cast::<InventorySlotUI>();
            {
                let mut slot_ui = slot_ui.bind_mut();
                slot_ui.item = slot.item.clone();
                slot_ui.quantity = slot.quantity;
            }
            if idx < hotbar_size {
                let spawn_point = self.hotbar_spawn_point_mut();
                spawn_point.add_child(&slot_ui);
                self.slots.push(slot_ui);
            } else {
                let spawn_point = self.inv_spawn_point_mut();
                spawn_point.add_child(&slot_ui);
                self.slots.push(slot_ui);
            }
        }
    }
}

impl InventoryUI {
    pub fn refresh(&mut self) {
        let slots = self.inventory().bind().get_slots();
        for (slot, slot_ui) in slots.iter_shared().zip(self.slots.iter_mut()) {
            let mut slot_ui = slot_ui.bind_mut();
            let slot = slot.bind();
            let mut diff = false;
            if slot_ui.item != slot.item {
                slot_ui.item = slot.item.clone();
                diff = true;
            }
            if slot_ui.quantity != slot.quantity {
                slot_ui.quantity = slot.quantity;
                diff = true;
            }
            if diff {
                slot_ui.refresh();
            }
        }
    }

    pub fn toggle(&mut self) {
        let inv_node = self.inventory_node_mut();
        let is_visible = inv_node.is_visible();
        inv_node.set_visible(!is_visible);
    }

    fn inventory_node(&self) -> &Gd<NinePatchRect> {
        self.inventory_node
            .as_ref()
            .expect("InventoryUI: inventory node is not set")
    }

    fn inventory_node_mut(&mut self) -> &mut Gd<NinePatchRect> {
        self.inventory_node
            .as_mut()
            .expect("InventoryUI: inventory node is not set")
    }

    pub fn inventory(&self) -> &Gd<Inventory> {
        self.inventory
            .as_ref()
            .expect("InventoryUI: inventory is not set")
    }

    fn inventory_mut(&mut self) -> &mut Gd<Inventory> {
        self.inventory
            .as_mut()
            .expect("InventoryUI: inventory is not set")
    }

    fn inv_spawn_point(&self) -> &Gd<GridContainer> {
        self.inv_spawn_point
            .as_ref()
            .expect("InventoryUI: inv_spawn_point is not set")
    }

    fn inv_spawn_point_mut(&mut self) -> &mut Gd<GridContainer> {
        self.inv_spawn_point
            .as_mut()
            .expect("InventoryUI: inv_spawn_point is not set")
    }

    fn hotbar_spawn_point(&self) -> &Gd<GridContainer> {
        self.hotbar_spawn_point
            .as_ref()
            .expect("InventoryUI: hotbar_spawn_point is not set")
    }

    fn hotbar_spawn_point_mut(&mut self) -> &mut Gd<GridContainer> {
        self.hotbar_spawn_point
            .as_mut()
            .expect("InventoryUI: hotbar_spawn_point is not set")
    }
}
