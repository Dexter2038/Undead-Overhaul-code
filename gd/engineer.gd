extends CharacterBody2D
class_name Player

enum State { IDLE, JUMP, MOVE }
enum Dir { LEFT = -1, RIGHT = 1 }

@export_group("Scalars")
@export var speed: float = 300.0
@export var jump_velocity: float = 100.0
@export var push_force: float = 10.0

@export_group("Nodes")
@export var flipper: Node2D
@export var hud: CanvasLayer
@export var sprite: Sprite2D
@export var anim_player: AnimationPlayer
@export var tool_marker: Marker2D

@export_group("Resources")
@export var inventory: Inventory

var inventory_ui: InventoryUI
var tool: Tool

var pick_items: Array = []
var state: State = State.IDLE
var dir: Dir = Dir.RIGHT

func _ready() -> void:
	if sprite == null:
		push_warning("Sprite node not found")
	if anim_player == null:
		push_warning("AnimationPlayer node not found")
	if inventory == null:
		push_warning("Inventory node not found")
	if flipper == null:
		push_warning("Flipper node not found")
	if hud == null:
		push_warning("HUD node not found")
	if tool_marker == null:
		push_warning("Tool marker not found")

	var inventory_ui_scene := load("res://scenes/ui/inventory.tscn") as PackedScene
	var instance = inventory_ui_scene.instantiate()
	if instance == null:
		push_warning("Failed to instantiate inventory UI")
		return

	inventory_ui = instance
	inventory_ui.inventory = inventory
	hud.add_child(inventory_ui)
	inventory_ui.slot_clicked.connect(on_slot_clicked)

func on_slot_clicked(idx: int) -> void:
	var slot: InventorySlot = inventory.slots.get(idx)
	if slot == null:
		return
	var item: InventoryItem = slot.item
	if item == null:
		return
	if !item.equippable:
		return
	if tool:
		tool.queue_free()
	var tool: Tool = item.equip_path.instantiate()
	tool.tilemap = get_node("..")
	tool_marker.add_child(tool)
	self.tool = tool

func _input(_event: InputEvent) -> void:
	pick_item()
	inv_toggle()

func _process(delta: float) -> void:
	movement(delta)

func pick_item() -> void:
	if Input.is_action_just_pressed("ui_pick"):
		if pick_items.is_empty():
			return

		var pickable_item = pick_items[0]
		if pickable_item == null:
			return

		var item = pickable_item.item
		var quantity = pickable_item.quantity
		var result = inventory.add_item(item, quantity)

		inventory_ui.refresh()

		if result == OK:
			pick_items.pop_front().queue_free()
			if pick_items.size() > 0:
				pick_items[0].enable_glow()
		else:
			pickable_item.quantity = result

func inv_toggle() -> void:
	if Input.is_action_just_pressed("ui_inv") and inventory_ui:
		inventory_ui.toggle()

func movement(delta: float) -> void:
	if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = -jump_velocity
		set_state(State.JUMP)
	else:
		var direction = Input.get_action_strength("ui_right") - Input.get_action_strength("ui_left")
		set_dir(direction)
		velocity.x = direction * speed

		if direction != 0.0:
			if is_on_floor():
				set_state(State.MOVE)
		else:
			velocity.x = move_toward(velocity.x, 0.0, speed)
			if is_on_floor():
				set_state(State.IDLE)

	if not is_on_floor():
		velocity.y += ProjectSettings.get_setting("physics/2d/default_gravity") * delta
	move_and_slide()
	
	for i in get_slide_collision_count():
		var c = get_slide_collision(i)
		if c.get_collider() is RigidBody2D:
			c.get_collider().apply_central_impulse(-c.get_normal() * push_force)

func set_state(new_state: State) -> void:
	if state == new_state:
		return
	state = new_state
	match state:
		State.IDLE: anim_player.play("idel")
		State.JUMP: anim_player.play("jump")
		State.MOVE: anim_player.play("move")

func set_dir(velocity_x: float) -> void:
	var new_dir: Dir
	if self.tool == null:
		if velocity_x > 0.0:
			new_dir = Dir.RIGHT
		elif velocity_x < 0.0:
			new_dir = Dir.LEFT
		else:
			return
	else:
		var mouse_x = get_local_mouse_position().x
		if mouse_x > 0.0:
			new_dir = Dir.RIGHT
		elif mouse_x < 0.0:
			new_dir = Dir.LEFT
		else:
			return

	if dir == new_dir:
		return
	dir = new_dir
	flipper.scale = Vector2(dir, 1.0)
