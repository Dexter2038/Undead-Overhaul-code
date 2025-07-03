extends Resource
class_name InventoryItem

@export var name: StringName
@export var icon_path: Texture2D
@export var icon_scale: float
@export var icon_offset: Vector2
@export var max_stack: int = 1
@export var equippable: bool
@export var equip_path: PackedScene

func icon() -> Texture2D:
	assert(icon_path != null, "icon_path must be set before calling icon()")
	return icon_path
