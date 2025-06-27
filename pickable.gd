extends RigidBody2D
#class_name Pickable

@export var item: InvItem
@export var count := 1
@export var sprite: Sprite2D
@export var area: Area2D
@onready var sprite_material: ShaderMaterial = sprite.material


func _ready() -> void:
	area.body_entered.connect(_on_area_2d_body_shape_entered)
	area.body_exited.connect(_on_area_2d_body_shape_exited)


func _on_area_2d_body_shape_entered(body: Node2D) -> void:
	if body is Player:
		var length := len(body.pick)
		if length:
			body.pick[0].disable_glow()
			pass
		enable_glow()
		body.pick.push_front(self)


func enable_glow() -> void:
	sprite_material.set_shader_parameter("enable_glow", true)


func disable_glow() -> void:
	sprite_material.set_shader_parameter("enable_glow", false)


func _on_area_2d_body_shape_exited(body: Node2D) -> void:
	if body is Player:
		disable_glow()
		body.pick.erase(self)
