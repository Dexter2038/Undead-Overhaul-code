extends Control
class_name InventoryUI

signal slot_clicked(slot: int)

@export var inventory_node: NinePatchRect
@export var inv_spawn_point: GridContainer
@export var hotbar_spawn_point: GridContainer
var inventory: Inventory
var slots: Array[InventorySlotUI] = []

func _ready() -> void:
	if inv_spawn_point == null:
		push_warning("InventoryUI: spawn point is not set")
	if hotbar_spawn_point == null:
		push_warning("InventoryUI: hotbar spawn point is not set")
	if inventory == null:
		push_warning("InventoryUI: inventory is not set")
	if inventory_node == null:
		push_warning("InventoryUI: inventory node is not set")

	inventory_node.visible = false

	var inv = inventory
	var all_slots :Array[InventorySlot]= inv.slots
	var hotbar_size :int = inv.hotbar_size

	if inv.size + inv.hotbar_size != all_slots.size():
		push_warning("InventoryUI: inventory size mismatch: %d != %d" % [
			inv.size + inv.hotbar_size,
			all_slots.size()
		])

	for idx in all_slots.size():
		var slot := all_slots[idx]
		var packed := preload("res://scenes/ui/inventory_slot.tscn")
		var slot_node: InventorySlotUI = packed.instantiate()
		if slot_node == null:
			push_error("Failed to instantiate inventory slot scene")
			return

		var slot_button := slot_node
		if slot_button:
			var captured_idx := idx
			slot_button.pressed.connect(func() -> void:
				emit_signal("slot_clicked", captured_idx)
			)

		var slot_ui := slot_node as InventorySlotUI
		if slot_ui:
			slot_ui.item = slot.item
			slot_ui.quantity = slot.quantity

			if idx < hotbar_size:
				hotbar_spawn_point.add_child(slot_ui)
			else:
				inv_spawn_point.add_child(slot_ui)

			slots.append(slot_ui)

func refresh() -> void:
	var all_slots := inventory.slots
	for i in all_slots.size():
		var slot := all_slots[i]
		var slot_ui := slots[i]
		var changed := false

		if slot_ui.item != slot.item:
			slot_ui.item = slot.item
			changed = true
		if slot_ui.quantity != slot.quantity:
			slot_ui.quantity = slot.quantity
			changed = true

		if changed:
			slot_ui.refresh()

func toggle() -> void:
	if inventory_node:
		inventory_node.visible = not inventory_node.visible
