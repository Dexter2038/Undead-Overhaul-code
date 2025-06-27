use godot::{
    classes::{
        AnimationPlayer, CharacterBody2D, ICharacterBody2D, Input, InputEvent, RigidBody2D,
        Sprite2D,
    },
    global::move_toward,
    prelude::*,
};

use crate::pickable::Pickable;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Idle,
    Jump,
    Move,
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
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
    sprite: Option<Gd<Sprite2D>>,

    #[export]
    anim_player: Option<Gd<AnimationPlayer>>,

    pub pick_items: Array<Gd<Pickable>>,

    #[init(val=State::Idle)]
    state: State,

    #[init(val=Dir::Right)]
    dir: Dir,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        if self.sprite.is_none() {
            panic!("Sprite node not found");
        };
        if self.anim_player.is_none() {
            panic!("AnimationPlayer node not found");
        };
    }

    fn input(&mut self, input: Gd<InputEvent>) {
        self.pick_item(&input);
        self.inv_toggle(&input);
    }

    fn physics_process(&mut self, delta: f32) {
        self.movement(delta);
    }
}

impl Player {
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

    fn pick_item(&mut self, input: &Gd<InputEvent>) {
        if input.is_action_pressed("ui_pick") {
            let len = self.pick_items.len();
            if len == 0 {
                return;
            }
            let Some(pickable_item) = self.pick_items.pop_front() else {
                return;
            };
            let pickable_item = pickable_item.bind();
            let item = pickable_item.item();
            let quantity = pickable_item.quantity;
            todo!("Pick up item")
        }
    }

    fn inv_toggle(&mut self, input: &Gd<InputEvent>) {
        if input.is_action_pressed("ui_inv") {
            todo!("Open inventory")
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
            let speed = self.speed;
            velocity.x = direction * speed;
            if direction != 0.0 {
                self.set_dir(if direction > 0.0 {
                    Dir::Right
                } else {
                    Dir::Left
                });
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
            let gravity = self.base().get_gravity();
            let gravity = Vector2::new(gravity.x * delta, gravity.y * delta);
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
                    let impulse =
                        Vector2::new(normal.x * self.push_force, normal.y * self.push_force);
                    obj.apply_impulse(impulse);
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

    fn set_dir(&mut self, dir: Dir) {
        if self.dir == dir {
            return;
        }
        self.dir = dir;
        let scale = self.base().get_scale();
        self.base_mut()
            .set_scale(Vector2::new(scale.x * -1.0, scale.y))
    }
}
