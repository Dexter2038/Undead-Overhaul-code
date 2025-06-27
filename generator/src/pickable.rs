use godot::{
    classes::{Area2D, IRigidBody2D, RigidBody2D, ShaderMaterial, Sprite2D},
    prelude::*,
};

use crate::{inventory::item::InventoryItem, player::Player};

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub struct Pickable {
    base: Base<RigidBody2D>,

    #[export]
    sprite: Option<Gd<Sprite2D>>,

    #[export]
    area: Option<Gd<Area2D>>,

    #[export]
    item: Option<Gd<InventoryItem>>,

    pub quantity: u32,
}

#[godot_api]
impl IRigidBody2D for Pickable {
    fn ready(&mut self) {
        if self.area.is_none() {
            panic!("Pickable must have an area");
        }
        if self.sprite.is_none() {
            panic!("Pickable must have a sprite");
        }
        if self.item.is_none() {
            panic!("Pickable must have an item");
        }
        if self.sprite().get_material().is_none() {
            godot_warn!("Pickable sprite has no material");
        }
        if self
            .sprite()
            .get_material()
            .is_some_and(|m| !m.is_class("ShaderMaterial"))
        {
            godot_warn!("Pickable sprite material is not a ShaderMaterial");
        }
        self.area()
            .signals()
            .body_entered()
            .connect_other(self, Self::on_body_entered);
        self.area()
            .signals()
            .body_exited()
            .connect_other(self, Self::on_body_exited);
    }
}

impl Pickable {
    #[allow(dead_code)]
    pub fn sprite(&self) -> &Gd<Sprite2D> {
        self.sprite.as_ref().unwrap()
    }

    #[allow(dead_code)]
    pub fn sprite_mut(&mut self) -> &mut Gd<Sprite2D> {
        self.sprite.as_mut().unwrap()
    }

    #[allow(dead_code)]
    pub fn area(&self) -> &Gd<Area2D> {
        self.area.as_ref().unwrap()
    }

    #[allow(dead_code)]
    pub fn area_mut(&mut self) -> &mut Gd<Area2D> {
        self.area.as_mut().unwrap()
    }

    pub fn item(&self) -> &Gd<InventoryItem> {
        self.item.as_ref().unwrap()
    }

    pub fn get_shader_material(&mut self) -> Option<Gd<ShaderMaterial>> {
        let material = self.sprite_mut().get_material()?;
        if !material.is_class("ShaderMaterial") {
            return None;
        }
        Some(material.cast())
    }

    pub fn enable_glow(&mut self) {
        let material = self.get_shader_material();
        if material.is_none() {
            return;
        }
        let mut material = material.unwrap();
        material.set_shader_parameter("enable_glow", &Variant::from(true));
    }

    pub fn disable_glow(&mut self) {
        let material = self.get_shader_material();
        if material.is_none() {
            return;
        }
        let mut material = material.unwrap();
        material.set_shader_parameter("enable_glow", &Variant::from(false));
    }

    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.is_class("Player") {
            let mut player: Gd<Player> = body.cast();
            let mut player = player.bind_mut();
            if let Some(mut item) = player.pick_items.front() {
                item.bind_mut().disable_glow();
            }
            self.enable_glow();
            player.pick_items.push_front(&self.to_gd());
        }
    }

    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        if body.is_class("Player") {
            let mut player: Gd<Player> = body.cast();
            self.disable_glow();
            let mut player = player.bind_mut();
            player.pick_items.erase(&self.to_gd());
        }
    }
}
