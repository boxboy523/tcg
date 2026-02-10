extends Unit
# Called when the node enters the scene tree for the first time.
@onready var health_bar = $HpBar
@onready var area = $Area2D
@onready var highlight = $HighlightCircle

const CELL_WIDTH = 128

signal clicked(unit_instance)

func _ready() -> void:
	hp_changed.connect(_on_hp_changed)	
	grid_index_changed.connect(_on_grid_index_changed)
	died.connect(_on_died)
	area.input_event.connect(_on_area_2d_input_event)

	position.x = grid_index * CELL_WIDTH

	health_bar.max_value = max_hp
	health_bar.value = hp

	if highlight: highlight.visible = false

func set_highlight(is_active: bool):
	if highlight:
		highlight.visible = is_active

		if is_active:
			var tween = create_tween().set_loops()
			tween.tween_property(highlight, "modulate:a", 0.5, 0.5)
			tween.tween_property(highlight, "modulate:a", 1.0, 0.5)
		else:
			highlight.modulate.a = 1.0

func _on_hp_changed(current_hp: int, max_hp: int):
	if health_bar:
		health_bar.max_value = max_hp
		health_bar.value = current_hp

func _on_died(_idx):
	print("유닛 사망!")

func _on_grid_index_changed(new_index):
	var tween = create_tween()
	tween.tween_property(self, "position:x", new_index * CELL_WIDTH, 0.2)

func _on_area_2d_input_event(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		clicked.emit(self)
