use godot::{
    classes::{
        AnimationPlayer, CanvasLayer, CharacterBody2D, ICharacterBody2D, Input, InputEvent,
        Marker2D, RigidBody2D, Sprite2D,
    },
    global::move_toward,
    prelude::*,
};

use crate::{
    drill::Tool,
    inventory::{inv::Inventory, ui::inv::InventoryUI},
    pickable::Pickable,
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Idle,
    Jump,
    Move,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn as_f32(self) -> f32 {
        match self {
            Dir::Left => -1.0,
            Dir::Right => 1.0,
        }
    }
}

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[export]
    #[init(val = 300.0)]
    speed: f32,

    #[export]
    #[init(val = 100.0)]
    jump_velocity: f32,

    #[export]
    #[init(val = 10.0)]
    push_force: f32,

    #[export]
    flipper: Option<Gd<Node2D>>,

    #[export]
    hud: Option<Gd<CanvasLayer>>,

    #[export]
    sprite: Option<Gd<Sprite2D>>,

    #[export]
    anim_player: Option<Gd<AnimationPlayer>>,

    #[export]
    tool_marker: Option<Gd<Marker2D>>,

    #[export]
    inventory: Option<Gd<Inventory>>,

    #[var]
    inventory_ui: Option<Gd<InventoryUI>>,

    pub pick_items: Array<Gd<Pickable>>,

    tool: Option<DynGd<Node2D, dyn Tool>>,

    #[init(val=State::Idle)]
    state: State,

    #[init(val=Dir::Right)]
    dir: Dir,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        if self.sprite.is_none() {
            godot_warn!("Sprite node not found");
        };
        if self.anim_player.is_none() {
            godot_warn!("AnimationPlayer node not found");
        };
        if self.inventory.is_none() {
            godot_warn!("Inventory node not found");
        };
        if self.flipper.is_none() {
            godot_warn!("Flipper node not found");
        }
        let Some(hud) = self.hud.as_mut() else {
            godot_warn!("HUD node not found");
            return;
        };
        let inventory_ui_scene = load::<PackedScene>("res://scenes/ui/inventory.tscn");
        let Some(inventory_ui) = inventory_ui_scene.instantiate() else {
            godot_warn!("Failed to instantiate inventory UI");
            return;
        };
        let mut inventory_ui = inventory_ui.cast::<InventoryUI>();
        {
            let mut ui_ref = inventory_ui.bind_mut();
            ui_ref.inventory = self.inventory.clone();
        }
        hud.add_child(&inventory_ui);

        inventory_ui
            .signals()
            .slot_clicked()
            .connect_other(self, Self::take_tool);

        self.inventory_ui = Some(inventory_ui);
    }

    fn input(&mut self, _input: Gd<InputEvent>) {
        self.pick_item();
        self.inv_toggle();
    }

    fn process(&mut self, delta: f32) {
        self.movement(delta);
    }
}

impl Player {
    fn take_tool(&mut self, idx: u32) {
        let slot = {
            let inventory = self.inventory().bind();
            if idx >= inventory.size + inventory.hotbar_size {
                return;
            }
            let Some(slot) = inventory.get_slots().get(idx as usize) else {
                return;
            };
            slot
        };
        let slot = slot.bind();
        let Some(item) = slot.item.as_ref() else {
            return;
        };
        let item = item.bind();
        if !item.equippable {
            return;
        }
        let Some(scene) = item.equip_path.clone() else {
            return;
        };
        let Some(scene) = scene.instantiate() else {
            return;
        };
        let Some(tool) = scene
            .cast::<Node2D>()
            .to_variant()
            .try_to::<DynGd<Node2D, dyn Tool>>()
            .ok()
        else {
            return;
        };
        self.tool_marker
            .as_mut()
            .unwrap()
            .add_child(&tool.to_godot());
        self.tool = Some(tool);
    }

    fn pick_item(&mut self) {
        if Input::singleton().is_action_just_pressed("ui_pick") {
            let len = self.pick_items.len();
            if len == 0 {
                return;
            }
            let Some(pickable_item) = self.pick_items.front() else {
                return;
            };
            let pickable_item = pickable_item.bind();
            let item = pickable_item.item();
            let quantity = pickable_item.quantity;
            let result = {
                let Some(inventory) = self.inventory.as_mut() else {
                    return;
                };
                let mut inventory = inventory.bind_mut();
                inventory.add_item(item, quantity)
            };
            self.inventory_ui_mut().bind_mut().refresh();

            // If item was successfully added to the inventory
            if result.is_ok() {
                if let Some(mut pickable_item) = self.pick_items.pop_front() {
                    pickable_item.queue_free();
                }
                if let Some(mut pickable_item) = self.pick_items.front() {
                    let mut pickable_item = pickable_item.bind_mut();
                    pickable_item.enable_glow();
                }
                return;
            }
            // If either item was not added to the inventory at all or only partially
            let Some(mut pickable_item) = self.pick_items.front() else {
                return;
            };
            let mut pickable_item = pickable_item.bind_mut();
            if let Err(quantity) = result {
                pickable_item.quantity = quantity;
            };
        }
    }

    fn inv_toggle(&mut self) {
        if Input::singleton().is_action_just_pressed("ui_inv") {
            let Some(inventory_ui) = self.inventory_ui.as_mut() else {
                return;
            };
            let mut inventory_ui = inventory_ui.bind_mut();
            inventory_ui.toggle();
        }
    }

    fn movement(&mut self, delta: f32) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();
        if input.is_action_just_pressed("ui_accept") && self.base().is_on_floor() {
            let jump_velocity = self.jump_velocity;
            velocity.y = -jump_velocity;
            self.set_state(State::Jump)
        } else {
            let direction = input.get_axis("ui_left", "ui_right");
            self.set_dir(direction);
            let speed = self.speed;
            velocity.x = direction * speed;
            if direction != 0.0 {
                if self.base().is_on_floor() {
                    self.set_state(State::Move);
                }
            } else {
                let x = move_toward(velocity.x.as_f64(), 0.0f64, speed.as_f64());
                velocity.x = x as f32;
                if self.base().is_on_floor() {
                    self.set_state(State::Idle);
                }
            }
        }
        if !self.base().is_on_floor() {
            let gravity = self.base().get_gravity() * delta;
            velocity += gravity
        }
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        for i in 0..self.base().get_slide_collision_count() {
            let c = self.base_mut().get_slide_collision(i);
            match c {
                Some(c) => {
                    let Some(col) = c.get_collider() else {
                        continue;
                    };
                    if !col.is_class("RigidBody2D") {
                        continue;
                    }
                    let mut obj: Gd<RigidBody2D> = col.cast();
                    let normal = -c.get_normal();
                    if normal.y.abs() > 0.7 {
                        continue;
                    }
                    let impulse =
                        Vector2::new(normal.x * self.push_force, normal.y * self.push_force);
                    obj.apply_central_impulse_ex().impulse(impulse).done();
                    //obj.apply_impulse(impulse);
                }
                None => continue,
            }
        }
    }

    fn set_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.state = state;
        match state {
            State::Idle => self.anim_player_mut().set_current_animation("idel"),
            State::Jump => self.anim_player_mut().set_current_animation("jump"),
            State::Move => self.anim_player_mut().set_current_animation("move"),
        }
    }

    fn set_dir(&mut self, velocity: f32) {
        let dir = if self.tool.is_none() {
            if velocity > 0.0 {
                Dir::Right
            } else if velocity < 0.0 {
                Dir::Left
            } else {
                return;
            }
        } else {
            let mouse_pos_x = self.base_mut().get_local_mouse_position().x;
            if mouse_pos_x > 0.0 {
                Dir::Right
            } else if mouse_pos_x < 0.0 {
                Dir::Left
            } else {
                return;
            }
        };
        if self.dir == dir {
            return;
        }
        self.dir = dir;
        self.flipper_mut()
            .set_scale(Vector2::new(dir.as_f32(), 1.0));
    }

    #[allow(dead_code)]
    fn sprite(&self) -> &Gd<Sprite2D> {
        self.sprite
            .as_ref()
            .expect("sprite must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn sprite_mut(&mut self) -> &mut Gd<Sprite2D> {
        self.sprite
            .as_mut()
            .expect("sprite must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn anim_player(&self) -> &Gd<AnimationPlayer> {
        self.anim_player
            .as_ref()
            .expect("anim_player must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn anim_player_mut(&mut self) -> &mut Gd<AnimationPlayer> {
        self.anim_player
            .as_mut()
            .expect("anim_player must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn inventory(&self) -> &Gd<Inventory> {
        self.inventory
            .as_ref()
            .expect("inventory must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn inventory_mut(&mut self) -> &mut Gd<Inventory> {
        self.inventory
            .as_mut()
            .expect("inventory must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn inventory_ui(&self) -> &Gd<InventoryUI> {
        self.inventory_ui
            .as_ref()
            .expect("inventory must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn inventory_ui_mut(&mut self) -> &mut Gd<InventoryUI> {
        self.inventory_ui
            .as_mut()
            .expect("inventory must be initialized in _ready()")
    }

    #[allow(dead_code)]
    fn flipper_mut(&mut self) -> &mut Gd<Node2D> {
        self.flipper
            .as_mut()
            .expect("flipper must be initialized in _ready()")
    }
}
