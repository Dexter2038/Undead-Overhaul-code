use godot::{classes::Texture2D, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct InventoryItem {
    base: Base<Resource>,

    #[export]
    name: StringName,

    #[export]
    pub icon_path: Option<Gd<Texture2D>>,

    #[export]
    pub icon_scale: f32,

    #[export]
    pub icon_offset: Vector2,

    #[export]
    #[init(val = 1)]
    pub max_stack: u32,
}

impl InventoryItem {
    #[allow(dead_code)]
    pub fn icon(&self) -> &Gd<Texture2D> {
        self.icon_path.as_ref().unwrap()
    }
}
