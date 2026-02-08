# Main.gd
extends Node2D

@onready var field = $Field
@onready var hand_ui = $CanvasLayer/HandUI

@export var unit_scene: PackedScene

var selected_card_index: int = -1

var card_strike = preload("res://cards/strike.tres") # 경로 맞춰주세요
var card_fireball = preload("res://cards/fireball.tres")

func _ready():
	var starter_deck: Array[Card] = []
	
	for i in range(5):
		starter_deck.append(card_strike)
	for i in range(2):
		starter_deck.append(card_fireball)
	
	field.initialize_deck(starter_deck)
	
	hand_ui.card_clicked.connect(_on_card_selected)
	
	for i in range(5):
		field.draw_card()

func _on_card_selected(index: int):
	# 같은 거 또 누르면 취소
	if selected_card_index == index:
		selected_card_index = -1
		print("선택 취소")
		return

	selected_card_index = index
	
	# 어떤 카드를 집었는지 로그 확인
	var card = field.hand[index] as Resource
	print("카드 선택됨: ", card.name, " (비용: ", card.cost, ")")

func _on_unit_clicked(target_unit):
	if selected_card_index == -1:
		return # 카드를 안 집고 유닛을 누르면 무시
	
	_execute_attack(target_unit)

func _execute_attack(target_unit):
	# A. 카드 정보 가져오기 (Rust Hand 배열에서)
	var card = field.hand[selected_card_index] as Resource
	
	# B. 사거리 체크 (간단하게 구현)
	# 실제로는 player_unit 위치와 target_unit 위치를 비교해야 함
	# if field.get_distance(player, target) > card.range:
	#     print("사거리가 안 닿습니다!")
	#     return

	print(card.name, " 발동! -> 대상: ", target_unit.name)

	# C. [중요] 데미지 적용
	# Rust에서 처리할 수도 있지만, 일단 Godot에서 처리
	target_unit.take_damage(card.damage) 
	# Unit.gd에 func take_damage(amount)가 있어야 함 (아래 참고)

	# D. [핵심] Rust에게 "카드 썼다"고 알림 (핸드 소각 로직 발동)
	field.play_card(selected_card_index)
	
	# E. UI 초기화
	selected_card_index = -1
	hand_ui.selected_card_index = -1 # UI 선택 표시 해제
	hand_ui._reset_all_cards_visual()
