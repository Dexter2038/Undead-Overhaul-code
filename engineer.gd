extends CharacterBody2D
#class_name Player


@export var sprite_2d: Sprite2D
@export var animation_player: AnimationPlayer
@export var inv: Inventory

@export var SPEED := 300.0
@export var JUMP_VELOCITY := -400.0
@export var PUSH_FORCE := 80.0

enum Dir {
	Left,
	Right
}

enum State {
	Idle,
	Move,
	Jump,
	Fall
}

var dir := Dir.Right
var state := State.Idle
var pick: Array[Pickable] = []


func set_dir(new_dir: Dir) -> void:
	if new_dir == dir:
		return
	dir = new_dir
	match dir:
		Dir.Right:
			scale.x *= -1
		Dir.Left:
			scale.x *= -1 


func set_state(new_state: State) -> void:
	if state == new_state:
		return
	state = new_state
	match state:
		State.Idle:
			animation_player.play("idel")
		State.Move:
			animation_player.play("move")
		State.Jump:
			animation_player.play("jump")
		State.Fall:
			animation_player.play("fall")


func _physics_process(delta: float) -> void:
	if Input.is_action_just_pressed("ui_pick"):
		var length := len(self.pick)
		if length:
			var pickable_item: Pickable = self.pick[0]
			var item: InvItem = pickable_item.item
			var count = pickable_item.count
			inv.add_item(item, count)
			pickable_item.queue_free()
			self.pick.pop_front()
			if length > 1:
				self.pick[0].enable_glow()
	if Input.is_action_just_pressed("ui_inv"):
		inv.visible = !inv.visible
	# Handle jump.
	if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = JUMP_VELOCITY
		set_state(State.Jump)
	else:
		# Get the input direction and handle the movement/deceleration.
		var direction := Input.get_axis("ui_left", "ui_right")
		if direction:
			velocity.x = direction * SPEED
			set_dir(Dir.Right if direction > 0 else Dir.Left)
			if is_on_floor():
				set_state(State.Move)
		else:
			velocity.x = move_toward(velocity.x, 0, SPEED)
			if is_on_floor():
				set_state(State.Idle)

	# Add the gravity.
	if not is_on_floor():
		velocity += get_gravity() * delta

	move_and_slide()
	
	for i in get_slide_collision_count():
		var c = get_slide_collision(i)
		if c.get_collider() is RigidBody2D:
			c.get_collider().apply_central_impulse(-c.get_normal() * PUSH_FORCE)
