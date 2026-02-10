use godot::prelude::*;

use crate::data::UID;

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

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct CardInstance {
    pub owner_uid: UID,

    #[export]
    pub base_card: Option<Gd<Card>>,

    #[export]
    pub cost: i32,

    #[export]
    pub damage: i32,

    #[export]
    pub range: i32,

    #[export]
    pub name: GString,

    base: Base<Node>,
}

#[godot_api]
impl CardInstance {
    #[func]
    pub fn init_state(&mut self, card: Gd<Card>, owner: i32) {
        let bind = card.bind();
        self.cost = bind.cost as i32;
        self.damage = bind.damage as i32;
        self.range = bind.range as i32;
        self.name = bind.name.clone();

        drop(bind);

        self.base_card = Some(card);
        self.owner_id = owner;

        self.base_mut().set_name(&format!("Card_{}", owner));
    }

    #[func]
    pub fn get_final_damage(&self) -> i32 {
        if let Some(card) = &self.base_card {
            return card.bind().damage as i32;
        }
        0
    }
}
