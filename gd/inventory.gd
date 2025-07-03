extends Resource
class_name Inventory

@export var size: int = 25
@export var hotbar_size: int = 3
@export var slots: Array[InventorySlot] = []

func expand(new_size: int) -> bool:
	if new_size <= size:
		return false

	size = new_size
	var total_slots = hotbar_size + size
	var new_slot := InventorySlot.new()
	while slots.size() < total_slots:
		slots.append(new_slot.duplicate())

	return true

func remove_from_slot(slot_index: int) -> bool:
	if slot_index < 0 or slot_index >= slots.size():
		return false

	var slot: InventorySlot = slots[slot_index]
	slot.item = null
	slot.quantity = 0
	return true

func add_item(item: InventoryItem, quantity: int) -> int:
	var max_stack := item.max_stack

	# First, fill existing stacks
	for slot in slots:
		if quantity == 0:
			break
		if slot.item and slot.item.get_name() == item.get_name():
			if slot.quantity < max_stack:
				var to_add = min(max_stack - slot.quantity, quantity)
				slot.quantity += to_add
				quantity -= to_add

	# Then, add to empty slots
	for slot in slots:
		if quantity == 0:
			break
		if slot.item == null:
			var to_add = min(max_stack, quantity)
			slot.item = item
			slot.quantity = to_add
			quantity -= to_add

	return quantity  # return 0 if fully added, >0 if remaining

func move_item(from_index: int, to_index: int) -> bool:
	if from_index < 0 or from_index >= slots.size():
		return false
	if to_index < 0 or to_index >= slots.size():
		return false

	var from_slot: InventorySlot = slots[from_index]
	var to_slot: InventorySlot = slots[to_index]

	var tmp_item = from_slot.item
	var tmp_quantity = from_slot.quantity

	from_slot.item = to_slot.item
	from_slot.quantity = to_slot.quantity

	to_slot.item = tmp_item
	to_slot.quantity = tmp_quantity

	return true
