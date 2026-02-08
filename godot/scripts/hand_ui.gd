extends HBoxContainer

# [설정] CardUI 씬을 에디터에서 드래그해서 넣어주세요
@export var card_ui_scene: PackedScene 

# [참조] Main 씬의 Field 노드 경로 (상황에 맞게 수정 필요)
# 예: Main 씬 구조가 Main -> [Field, CanvasLayer -> HandUI] 라면
@onready var field = $"../../Field"

signal card_clicked(index)

# 현재 선택된(클릭된) 카드 인덱스
var selected_card_index: int = -1

func _ready():
	# Rust에서 핸드가 변경되었다고 신호를 보내면 UI 갱신
	if field:
		field.hand_updated.connect(_update_hand_ui)
		# 게임 시작 시 초기화가 필요하다면 호출
		# _update_hand_ui()
	else:
		push_error("Field node not found! Check the path.")

# --- 핸드 렌더링 (Rust 데이터 동기화) ---
func _update_hand_ui():
	# 1. 기존 카드 싹 지우기 (초기화)	
	for child in get_children():
		child.queue_free()
	
	selected_card_index = -1 # 선택 상태 초기화
	
	# 2. Rust의 hand 배열을 가져와서 카드 생성
	var hand_data = field.hand # Array<Gd<Card>>
	
	for i in range(hand_data.size()):
		var card_res = hand_data[i]
		
		# 카드 인스턴스 생성
		var card_instance = card_ui_scene.instantiate()
		add_child(card_instance)
		
		# 데이터 주입 및 시그널 연결
		card_instance.setup(i, card_res)
		
		# 카드에서 올라오는 이벤트 연결
		card_instance.hovered.connect(_on_card_hovered)
		card_instance.unhovered.connect(_on_card_unhovered)
		card_instance.clicked.connect(_on_card_clicked)

# --- 마우스 호버 효과 (Burn Preview) ---
func _on_card_hovered(index: int):
	# 이미 카드를 선택(클릭)해서 조준 중이라면 호버 효과 무시
	if selected_card_index != -1: 
		return

	# 1. 모든 카드 시각적 초기화
	_reset_all_cards_visual()
	
	# 2. 호버된 카드는 '사용 예정(초록색)' 표시
	var cards = get_children()
	if index < cards.size():
		cards[index].set_state_selected() # 혹은 set_state_hovered()
	
	# 3. [핵심] Rust에게 "이거 쓰면 누구 타나요?" 물어보기
	var burn_indices = field.get_burn_indices(index) # Rust 함수 호출
	
	# 4. 타버릴 카드들은 '소각 예정(빨간색)' 표시
	for burn_idx in burn_indices:
		if burn_idx < cards.size():
			cards[burn_idx].set_state_burn_target()

func _on_card_unhovered(index: int):
	# 선택 중이 아닐 때만 초기화
	if selected_card_index == -1:
		_reset_all_cards_visual()

func _reset_all_cards_visual():
	for card in get_children():
		card.reset_visual()

# --- 카드 클릭 (선택/취소) ---
func _on_card_clicked(index: int):
	if selected_card_index == index:
		selected_card_index = -1
		_reset_all_cards_visual()
		_on_card_hovered(index) 
	else:
		selected_card_index = index
		_on_card_hovered(index)
	
	card_clicked.emit(index)
	
