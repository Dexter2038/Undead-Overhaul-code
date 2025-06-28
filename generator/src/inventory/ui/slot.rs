use godot::{
    classes::{ITextureButton, Label, TextureButton, TextureRect},
    prelude::*,
};

use crate::inventory::item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base=TextureButton)]
pub struct InventorySlotUI {
    base: Base<TextureButton>,
    #[export]
    texture: Option<Gd<TextureRect>>,
    #[export]
    label: Option<Gd<Label>>,
    pub item: Option<Gd<InventoryItem>>,
    pub quantity: u32,
}

#[godot_api]
impl ITextureButton for InventorySlotUI {
    fn ready(&mut self) {
        if self.texture.is_none() {
            godot_warn!("InventorySlotUI: texture is not set");
        }
        if self.label.is_none() {
            godot_warn!("InventorySlotUI: label is not set");
        }
        self.refresh();
    }
}

impl InventorySlotUI {
    pub fn refresh(&mut self) {
        let item = self.item.clone();
        let quantity = self.quantity;
        match item {
            Some(ref item) => {
                let item = item.bind();
                let texture = self.texture_mut();
                texture.set_scale(Vector2::new(item.icon_scale, item.icon_scale));
                texture.set_pivot_offset(item.icon_offset);
                texture.set_texture(item.icon());
                texture.set_visible(true);
                let label = self.label_mut();
                if quantity > 1 {
                    label.set_text(format!("{}", quantity).as_str());
                } else {
                    label.set_text("")
                }
            }
            None => {
                let texture = self.texture_mut();
                texture.set_scale(Vector2::new(1.0, 1.0));
                texture.set_pivot_offset(Vector2::new(0.0, 0.0));
                texture.set_visible(false);
                let label = self.label_mut();
                label.set_text("");
            }
        }
    }

    fn texture(&self) -> &Gd<TextureRect> {
        self.texture
            .as_ref()
            .expect("InventorySlotUI: texture is not set")
    }

    fn texture_mut(&mut self) -> &mut Gd<TextureRect> {
        self.texture
            .as_mut()
            .expect("InventorySlotUI: texture is not set")
    }

    fn label(&self) -> &Gd<Label> {
        self.label
            .as_ref()
            .expect("InventorySlotUI: label is not set")
    }

    fn label_mut(&mut self) -> &mut Gd<Label> {
        self.label
            .as_mut()
            .expect("InventorySlotUI: label is not set")
    }
}
