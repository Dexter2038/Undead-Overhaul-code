extends RigidBody2D
class_name Pickable

@export var sprite: Sprite2D
@export var area: Area2D
@export var item: InventoryItem
@export var quantity: int = 0

func _ready() -> void:
	if area == null:
		push_warning("Pickable must have an area")
	if sprite == null:
		push_warning("Pickable must have a sprite")
	if item == null:
		push_warning("Pickable must have an item")

	var material = sprite.material
	if material == null:
		push_warning("Pickable sprite has no material")
	elif not material is ShaderMaterial:
		push_warning("Pickable sprite material is not a ShaderMaterial")

	area.body_entered.connect(on_body_entered)
	area.body_exited.connect(on_body_exited)

func get_shader_material() -> ShaderMaterial:
	var material = sprite.material
	if material and material is ShaderMaterial:
		return material
	return null

func enable_glow() -> void:
	var mat = get_shader_material()
	if mat != null:
		mat.set_shader_parameter("enable_glow", true)

func disable_glow() -> void:
	var mat = get_shader_material()
	if mat != null:
		mat.set_shader_parameter("enable_glow", false)

func on_body_entered(body: Node) -> void:
	if body is Player:
		var player: Player = body
		if player.pick_items.size() > 0:
			player.pick_items[0].disable_glow()
		enable_glow()
		player.pick_items.insert(0, self)

func on_body_exited(body: Node) -> void:
	if body is Player:
		disable_glow()
		var player: Player = body
		player.pick_items.erase(self)
