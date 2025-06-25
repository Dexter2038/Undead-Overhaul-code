extends CanvasLayer

signal InvSizeChanged

@export var spawn_point: GridContainer
var count: int = 25

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	for i in 25:
		var inv_item = preload("res://scenes/ui/inventory_item.tscn").instantiate()
		inv_item.id = i
		inv_item.name = "InvItem" + str(i)
		spawn_point.add_child(inv_item)
