extends PanelContainer

# 상위(HandUI)로 보낼 신호들
signal hovered(index)
signal unhovered(index)
signal clicked(index)

var card_index: int = -1
var card_data: Resource # Rust의 Card 리소스

# 노드 참조 (이름이 다르면 에디터에 맞게 수정하세요)
@onready var name_label = $Content/TopBar/NameLabel
@onready var cost_label = $Content/TopBar/CostLabel
@onready var desc_label = $Content/DescLabel
@onready var border = $Border
@onready var background = $Background

func _ready():
	# UI 초기화
	reset_visual()
	
	# 마우스 이벤트 연결 (PanelContainer는 GUI Input을 받음)
	gui_input.connect(_on_gui_input)
	mouse_entered.connect(_on_mouse_entered)
	mouse_exited.connect(_on_mouse_exited)

# 데이터 주입 함수 (HandUI에서 호출)
func setup(index: int, card: Resource):
	card_index = index
	card_data = card
	
	name_label.text = card.name
	cost_label.text = str(card.cost)
	
	# 데미지나 범위 같은 설명 표시
	var desc = "Rng: %d" % card.range
	if card.damage > 0:
		desc += "\nDmg: %d" % card.damage
	desc_label.text = desc

# --- 시각적 상태 변경 함수들 ---

func reset_visual():
	modulate = Color(1, 1, 1) # 원래 색
	border.border_color = Color(0, 0, 0, 0) # 테두리 숨김
	border.editor_only = false # 게임 중에도 보이게 설정 필요하면 켜기

func set_state_selected():
	# 선택됨: 초록색 틴트 + 두꺼운 테두리
	modulate = Color(0.7, 1.0, 0.7) 
	border.border_color = Color(0, 1, 0, 1) 
	border.border_width = 4.0

func set_state_burn_target():
	# 탈 예정: 붉은색 틴트 + 빨간 테두리
	modulate = Color(1.0, 0.5, 0.5)
	border.border_color = Color(1, 0, 0, 1)
	border.border_width = 2.0

# --- 입력 처리 ---

func _on_gui_input(event):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		clicked.emit(card_index)

func _on_mouse_entered():
	hovered.emit(card_index)
func _on_mouse_exited():
	unhovered.emit(card_index)
	
