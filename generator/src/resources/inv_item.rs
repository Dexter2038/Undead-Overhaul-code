use godot::{classes::Texture2D, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct InvItem {
    base: Base<Resource>,

    #[export]
    icon_path: Option<Gd<Texture2D>>,

    #[export]
    icon_scale: f32,

    #[export]
    icon_offset: Vector2,

    #[export]
    #[init(val = 1)]
    max_stack: u32,
}

impl InvItem {
    #[allow(dead_code)]
    fn icon(&self) -> &Gd<Texture2D> {
        self.icon_path.as_ref().unwrap()
    }
}
