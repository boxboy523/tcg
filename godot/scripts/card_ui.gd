extends PanelContainer

# 상위(HandUI)로 보낼 신호들
signal hovered(index)
signal unhovered(index)
signal clicked(index)

var card_index: int = -1
var card_data: CardInstance # Rust의 Card 리소스

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
func setup(index: int, card: CardInstance):
	card_index = index
	card_data = card

	name_label.text = card.name
	cost_label.text = str(card.cost)

	# 데미지나 범위 같은 설명 표시
	var desc = "Rng: %d" % card.range
	if card.damage > 0:
		desc += "\nDmg: %d" % card.damage
	desc_label.text = desc
	set_owner_visuals(card.owner_id)
# --- 시각적 상태 변경 함수들 ---

func set_owner_visuals(owner_id: int):
	# 예시: 0번 유닛(플레이어) = 파란색, 1번 유닛(동료) = 초록색, 그 외(적) = 빨간색
	var style = background.get_theme_stylebox("panel").duplicate() as StyleBoxFlat

	# 스타일박스가 없으면 새로 생성 (안전장치)
	if not style:
		style = StyleBoxFlat.new()
		style.set_corner_radius_all(8) # 둥근 모서리
		background.add_theme_stylebox_override("panel", style)

	# 색상 팔레트 정의
	match owner_id:
		0: # 플레이어 (파란색 계열)
			style.bg_color = Color(0.2, 0.3, 0.6, 1.0) 
		1: # 동료/용병 (청록색/보라색 계열)
			style.bg_color = Color(0.2, 0.5, 0.4, 1.0)
		_: # 적 (붉은색 계열)
			style.bg_color = Color(0.6, 0.2, 0.2, 1.0)

	background.add_theme_stylebox_override("panel", style)

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

