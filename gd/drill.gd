extends Tool

enum State { Active, Nonactive}

@export var hover: ColorRect
@export var anim_player: AnimationPlayer
@export var ray: RayCast2D

const tiles_deny: Array[Vector2i] = [
	Vector2i(11, 0), Vector2i(10, 0), Vector2i(9, 0), Vector2i(-1, -1)
]

var state := State.Nonactive

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	hover.visible = false
	if tilemap == null:
		push_warning("Drill: Tilemap is not set")
	if hover == null:
		push_warning("Drill: Hover is not set")
	if anim_player == null:
		push_warning("Drill: Animation Player is not set")
	if ray == null:
		push_warning("Drill: RayCast2D is not set")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var mouse_pos = get_global_mouse_position()
	look_at(mouse_pos)
	var map_pos = to_map(mouse_pos)
	var tilemap_pos = map_pos / 9
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		set_state(State.Active)
	else:
		set_state(State.Nonactive)
	if state == State.Active:
		var col = ray.get_collider()
		if col:
			var tilemap_col_pos = to_map(ray.get_collision_point()) / 9
			tilemap.set_cell(0, tilemap_col_pos)
	if tilemap.get_cell_atlas_coords(0, tilemap_pos) in tiles_deny:
		hover.visible = false
		return
	hover.visible = true
	hover.global_position = map_pos
	if state == State.Nonactive:
		return
	tilemap.set_cell(0, tilemap_pos)


func set_state(state: State) -> void:
	if state == self.state:
		return
	self.state = state
	match state:
		State.Active:
			anim_player.play("default")
		State.Nonactive:
			anim_player.stop()


func to_map(mouse_pos: Vector2i) -> Vector2i:
	var x = (mouse_pos.x)
	var y = (mouse_pos.y)
	var snap_x = (x/9) * 9
	if x < 0:
		snap_x -= 9
	var snap_y = (y/9)* 9
	if y < 0:
		snap_y -= 9
	return Vector2(snap_x, snap_y)
	#mouse_pos.y % 9
