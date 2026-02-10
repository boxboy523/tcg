# Main.gd
extends Node2D

@onready var field = $Field
@onready var hand_ui = $CanvasLayer/HandUI
@onready var range_indicator = $RangeLine

@export var unit_scene: PackedScene

var selected_card_index: int = -1

var current_player_grid_index: int = 0

var card_strike = preload("res://cards/strike.tres") # 경로 맞춰주세요
var card_fireball = preload("res://cards/fireball.tres")

func _ready():
	var starter_deck: Array[Card] = []
	var starter_deck_owner: Array[int] = []

	for i in range(5):
		starter_deck.append(card_strike)
		starter_deck_owner.append(0)
	for i in range(2):
		starter_deck.append(card_fireball)
		starter_deck_owner.append(1)
	field.initialize_deck(starter_deck, starter_deck_owner)

	field.spawn_unit(unit_scene, 0)
	field.spawn_unit(unit_scene, 2)

	_connect_units()

	hand_ui.card_clicked.connect(_on_card_selected)
	hand_ui.card_hovered_start.connect(_on_card_hover_start)
	hand_ui.card_hovered_end.connect(_on_card_hover_end)

	for i in range(5):
		field.draw_card()

func _connect_units():
	for child in field.get_children():
		if child is Unit: # Rust 클래스 체크
			if not child.clicked.is_connected(_on_unit_clicked):
				child.clicked.connect(_on_unit_clicked)

func _on_card_selected(index: int):
	if selected_card_index == index:
		selected_card_index = -1
		hand_ui.selected_card_index = -1
		print("선택 취소")
		return

	selected_card_index = index

	# 어떤 카드를 집었는지 로그 확인
	var card = field.hand[index]
	print("카드 선택됨: ", card.name, " (비용: ", card.cost, ")")

func _on_unit_clicked(target_unit):
	if selected_card_index == -1:
		print("카드를 먼저 선택하세요.")
		return

	_execute_attack(target_unit)

func _execute_attack(target_unit):
	var target_idx = target_unit.grid_index # Unit.rs에 export된 속성

	print("공격 시도! Card: %d, Source: %d, Target: %d" % [selected_card_index, current_player_grid_index, target_idx])

	# Rust 함수 호출
	var error = field.play_card(selected_card_index, current_player_grid_index, target_idx)

	if error == OK:
		print("공격 성공!")
		# 성공했으면 선택 상태 초기화
		selected_card_index = -1
		hand_ui.selected_card_index = -1
		hand_ui._reset_all_cards_visual()
	else:
		print("공격 실패! Error Code: ", error)

func _on_card_hover_start(card_instance):
	var owner_id = card_instance.owner_id
	var range_val = 0

	# Rust CardInstance에서 사거리 정보 가져오기 (export 필요, 없다면 card resource에서)
	range_val = card_instance.range # Card.rs에 range 필드 있다고 가정

	# 1. 주인 유닛 찾아서 강조 켜기
	var owner_unit = _find_unit_by_id(owner_id)
	if owner_unit:
		owner_unit.set_highlight(true)

		# 2. 사거리 표시 그리기
		_draw_range_indicator(owner_unit, range_val)

func _on_card_hover_end():
	# 모든 유닛 강조 끄기
	for child in field.get_children():
		if child is Unit: # Rust 클래스 체크
			child.set_highlight(false)

	# 사거리 표시 끄기
	range_indicator.clear_points()

# 유닛 ID로 실제 노드 찾는 헬퍼 함수
func _find_unit_by_id(id: int):
	for child in field.get_children():
		if child is Unit and child.grid_index == id:
			return child
	return null

# 사거리 그리기 (Line2D 사용 예시)
func _draw_range_indicator(unit, range_val):
	print("사거리 표시: 유닛 ID=", unit.grid_index, " 사거리=", range_val)
	range_indicator.clear_points()

	var center_global = unit.global_position
	center_global += Vector2(-64, 50)

	var cell_width = 128 # Unit.gd의 CELL_WIDTH와 맞춰주세요

	# 사거리가 1이면 앞뒤 1칸, 2면 2칸...
	# 시각적으로 바닥에 선을 긋습니다.
	var left_limit = center_global - Vector2(range_val * cell_width, 0)
	var right_limit = center_global + Vector2((range_val + 1) * cell_width, 0)

	# 빨간색 선으로 표시
	range_indicator.add_point(left_limit)
	range_indicator.add_point(right_limit)
	range_indicator.width = 10
	range_indicator.default_color = Color(1, 0, 0, 0.5)
