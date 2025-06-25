extends TextureButton

signal ItemChanged(item: InvItem, count: int)
signal CountChanged(count: int)

@export var texture: TextureRect
@export var label: Label

var id: int
var item: InvItem = null
var count: int = 0

func update_item(item: InvItem, count: int) -> void:
	self.item = item
	self.count = count
	if item:
		texture.scale = Vector2(item.scale, item.scale)
		texture.pivot_offset = Vector2(item.offset)
		texture.texture = item.icon
		label.text = str(count) if count > 1 else ""
	else:
		label.text = ""

func update_count(count: int) -> void:
	self.count = count
	label.text = str(count) if count > 1 else ""

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	ItemChanged.connect(_on_item_changed)
	CountChanged.connect(_on_count_changed)

func _on_item_changed(item: InvItem, count: int):
	update_item(item, count)

func _on_count_changed(count: int):
	update_count(count)
