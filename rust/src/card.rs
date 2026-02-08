use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Card {
    #[export]
    pub name: GString,
    #[export]
    pub cost: i32,
    #[export]
    pub damage: i32, // 0이면 데미지 없는 카드
    #[export]
    pub range: i32, // 사거리 (0이면 글로벌/즉발)

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Card {
    fn init(base: Base<Resource>) -> Self {
        Self {
            name: "New Card".into(),
            cost: 1,
            damage: 0,
            range: 1,
            base,
        }
    }
}
