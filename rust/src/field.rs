use godot::global::Error;
use godot::prelude::*;
use std::collections::BTreeMap;

use crate::card::{Card, CardInstance};
use crate::data::{GridIdx, UID};
use crate::unit::Unit;

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct Field {
    units: BTreeMap<UID, Gd<Unit>>,
    grid_map: BTreeMap<GridIdx, UID>,

    #[export]
    deck: Array<Gd<CardInstance>>,
    #[export]
    hand: Array<Gd<CardInstance>>,
    #[export]
    discard_pile: Array<Gd<CardInstance>>,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Field {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            units: BTreeMap::new(),
            grid_map: BTreeMap::new(),
            deck: Array::new(),
            hand: Array::new(),
            discard_pile: Array::new(),
            base,
        }
    }
}

#[godot_api]
impl Field {
    #[func]
    fn spawn_unit(&mut self, unit_scene: Option<Gd<PackedScene>>, grid_idx: GridIdx) -> UID {
        let Some(scene) = unit_scene else {
            godot_error!("Unit scene is None!");
            return UID::null();
        };

        let mut instance = scene.instantiate_as::<Unit>();
        let uid = UID::new();
        let grid_pos = grid_idx;
        {
            let mut bind = instance.bind_mut();
            bind.setup(grid_pos, uid);
        }
        let callable = self.base().callable("_on_unit_died");
        instance.connect("died", &callable);

        self.units.insert(uid, instance.clone());
        self.grid_map.insert(grid_pos, uid);
        self.base_mut().add_child(&instance.upcast::<Node>());

        uid
    }

    #[func]
    pub fn move_unit(&mut self, uid: UID, new_idx: GridIdx) -> Error {
        if self.grid_map.contains_key(&new_idx) {
            godot_print!("이동 불가: {}번 위치에 이미 유닛이 있음", new_idx.0);
            return Error::ERR_ALREADY_IN_USE;
        }

        if let Some(unit) = self.units.get_mut(&uid) {
            let mut bind = unit.bind_mut();
            let old_grid_index = bind.grid_index;

            bind.grid_index = new_idx;
            drop(bind);

            self.grid_map.remove(&old_grid_index);
            self.grid_map.insert(new_idx, uid);

            unit.emit_signal("grid_index_changed", &[new_idx.0.to_variant()]);

            return Error::OK;
        }

        Error::ERR_DOES_NOT_EXIST
    }

    #[func]
    fn clear(&mut self) {
        for (_, unit) in self.units.iter_mut() {
            unit.queue_free();
        }
        self.units.clear();
        self.grid_map.clear();
        self.deck.clear();
        self.hand.clear();
        self.discard_pile.clear();
    }

    #[func]
    fn _on_unit_died(&mut self, uid: UID) {
        godot_print!("Unit uid {} is dead.", uid.get());

        if let Some(mut unit) = self.units.remove(&uid) {
            let grid_index = unit.bind().grid_index;
            self.grid_map.remove(&grid_index);
            unit.queue_free();
        }
    }

    fn get_unit(&mut self, uid: UID) -> Option<Gd<Unit>> {
        match self.units.get_mut(&uid) {
            Some(u) => Some(u.clone()),
            None => None,
        }
    }

    #[func]
    pub fn initialize_deck(&mut self, starter_deck: Array<Gd<Card>>, owners: Array<i64>) {
        for mut card in self.deck.iter_shared() {
            card.queue_free();
        }
        for mut card in self.hand.iter_shared() {
            card.queue_free();
        }
        for mut card in self.discard_pile.iter_shared() {
            card.queue_free();
        }

        self.deck.clear();
        self.hand.clear();
        self.discard_pile.clear();

        for (card_res, owner) in starter_deck.iter_shared().zip(owners.iter_shared()) {
            let mut instance = CardInstance::new_alloc();
            {
                let mut bind = instance.bind_mut();
                bind.init_state(card_res, UID::from(owner as u32));
            }
            self.deck.push(&instance);
            self.base_mut().add_child(&instance.upcast::<Node>());
        }
        self.shuffle_deck();
    }

    #[func]
    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    #[func]
    pub fn draw_card(&mut self) {
        if self.deck.is_empty() {
            if self.discard_pile.is_empty() {
                godot_print!("No cards left to draw!");
                return;
            }
            self.deck.extend_array(&self.discard_pile);
            self.discard_pile.clear();
            self.shuffle_deck();
        }

        if let Some(card) = self.deck.pop() {
            self.hand.push(&card);

            self.base_mut().emit_signal("hand_updated", &[]);
        }
    }

    #[func]
    pub fn get_burn_indices(&self, play_idx: i64) -> Array<i64> {
        let mut burn_indices = Array::new();
        let play_idx = play_idx as usize;

        if play_idx >= self.hand.len() {
            return burn_indices;
        }

        let cost = self.hand.at(play_idx).bind().cost as usize;
        let mut collected = 0;

        for i in (0..self.hand.len()).rev() {
            if collected >= cost {
                break;
            }
            // 나 자신은 태우지 않음
            if i == play_idx {
                continue;
            }
            burn_indices.push(i as i64);
            collected += 1;
        }

        burn_indices
    }

    #[func]
    pub fn play_card(&mut self, card_idx: i64, target_uid: UID) -> Error {
        // 1. 유효성 검사 (인덱스 범위)
        if card_idx as usize >= self.hand.len() {
            return Error::ERR_INVALID_PARAMETER;
        }

        let mut indices_to_remove = self.get_burn_indices(card_idx);

        // 사용할 카드 임시 확보 (아직 제거 안 함)
        let card_to_play = self.hand.at(card_idx as usize).clone();
        let card_owner = self.units.get(&card_to_play.bind().owner_uid).cloned();
        let card_bind = card_to_play.bind();

        if card_bind.cost as usize > indices_to_remove.len() {
            godot_print!(
                "Not enough cards to burn! Need {}, have {}.",
                card_bind.cost,
                indices_to_remove.len()
            );
            return Error::ERR_UNAVAILABLE;
        }

        let mut target_unit = match self.get_unit(target_uid) {
            Some(u) => u,
            None => return Error::ERR_DOES_NOT_EXIST,
        };

        let dist = if let Some(owner_unit) = card_owner {
            let owner_idx = owner_unit.bind().grid_index;
            let target_idx = target_unit.bind().grid_index;
            GridIdx::dist(owner_idx, target_idx)
        } else {
            godot_print!("Card owner unit not found!");
            return Error::ERR_DOES_NOT_EXIST;
        };
        if dist > card_bind.range {
            godot_print!("Too far! Dist: {}, Range: {}", dist, card_bind.range);
            return Error::ERR_UNAVAILABLE;
        }

        if card_bind.damage > 0 {
            target_unit.bind_mut().take_damage(card_bind.damage);
        }

        indices_to_remove.push(card_idx);

        indices_to_remove.sort_unstable();
        indices_to_remove.reverse();

        for idx in indices_to_remove.iter_shared() {
            let removed_card = self.hand.remove(idx as usize);
            self.discard_pile.push(&removed_card);
        }

        godot_print!(
            "Played '{}' (Cost {}).",
            card_to_play.bind().name,
            card_to_play.bind().cost
        );

        self.base_mut().emit_signal("hand_updated", &[]);

        Error::OK
    }

    #[signal]
    fn hand_updated();
}
