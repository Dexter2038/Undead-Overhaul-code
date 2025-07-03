extends TextureButton
class_name InventorySlotUI

@export var texture: TextureRect
@export var label: Label

var item: InventoryItem = null
var quantity: int = 0

func _ready() -> void:
	if texture == null:
		push_warning("InventorySlotUI: texture is not set")
	if label == null:
		push_warning("InventorySlotUI: label is not set")
	refresh()

func refresh() -> void:
	if item:
		texture.scale = Vector2(item.icon_scale, item.icon_scale)
		texture.pivot_offset = item.icon_offset
		texture.texture = item.icon()
		texture.visible = true

		if quantity > 1:
			label.text = str(quantity)
		else:
			label.text = ""
	else:
		texture.scale = Vector2(1.0, 1.0)
		texture.pivot_offset = Vector2.ZERO
		texture.visible = false
		label.text = ""
