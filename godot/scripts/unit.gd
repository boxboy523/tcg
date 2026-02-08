extends Unit
# Called when the node enters the scene tree for the first time.
@onready var health_bar = $HpBar
@onready var area = $Area2D

const CELL_WIDTH = 128

signal clicked(unit_instance)

func _ready() -> void:
	hp_changed.connect(_on_hp_changed)	
	grid_index_changed.connect(_on_grid_index_changed)
	area.input_event.connect(_on_area_2d_input_event)

func _on_hp_changed(current_hp: int, max_hp: int):
	if health_bar:
		health_bar.max_value = max_hp
		health_bar.value = current_hp

func _on_grid_index_changed(new_index):
	var tween = create_tween()
	tween.tween_property(self, "position:x", new_index * CELL_WIDTH, 0.2)

func _on_area_2d_input_event(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		clicked.emit(self)
