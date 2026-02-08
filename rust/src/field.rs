use godot::global::Error;
use godot::prelude::*;
use std::collections::BTreeMap;

use crate::card::Card;
use crate::unit::Unit;

#[derive(GodotClass, Debug)]
#[class(base=Node2D, init)]
pub struct Field {
    units: BTreeMap<i32, Gd<Unit>>,
    #[export]
    deck: Array<Gd<Card>>,
    #[export]
    hand: Array<Gd<Card>>,
    #[export]
    discard_pile: Array<Gd<Card>>,

    base: Base<Node2D>,
}

#[godot_api]
impl Field {
    #[func]
    fn spawn_unit(&mut self, unit_scene: Option<Gd<PackedScene>>, grid_pos: i32) -> Error {
        let Some(scene) = unit_scene else {
            return Error::ERR_DOES_NOT_EXIST;
        };

        if self.units.contains_key(&grid_pos) {
            return Error::ERR_ALREADY_IN_USE;
        }

        let mut instance = scene.instantiate_as::<Unit>();

        let callable = self.base().callable("_on_unit_died");
        instance.connect("died", &callable);

        let (hp, max_hp) = {
            let bind = instance.bind();
            (bind.hp, bind.max_hp)
        };

        instance.emit_signal("hp_changed", &[hp.to_variant(), max_hp.to_variant()]);

        instance.bind_mut().update_grid_index(grid_pos);
        self.units.insert(grid_pos, instance.clone());
        self.base_mut().add_child(&instance.upcast::<Node>());

        Error::OK
    }

    #[func]
    fn move_unit(&mut self, from_idx: i32, to_idx: i32) -> Error {
        if self.units.contains_key(&to_idx) {
            return Error::ERR_ALREADY_IN_USE;
        }
        let mut unit = match self.units.remove(&from_idx) {
            Some(unit) => unit,
            None => {
                return Error::ERR_DOES_NOT_EXIST;
            }
        };
        unit.bind_mut().update_grid_index(to_idx);
        self.units.insert(to_idx, unit);
        Error::OK
    }

    #[func]
    fn clear(&mut self) {
        for (_, unit) in self.units.iter_mut() {
            unit.queue_free();
        }
        self.units.clear()
    }

    #[func]
    fn _on_unit_died(&mut self, grid_pos: i32) {
        godot_print!("Unit at {} is dead.", grid_pos);

        if let Some(mut unit) = self.units.remove(&grid_pos) {
            unit.queue_free();
        }
    }

    fn get_unit(&mut self, idx: i32) -> Option<Gd<Unit>> {
        match self.units.get_mut(&idx) {
            Some(u) => Some(u.clone()),
            None => None,
        }
    }

    #[func]
    pub fn initialize_deck(&mut self, starter_deck: Array<Gd<Card>>) {
        self.deck.clear();
        self.hand.clear();
        self.discard_pile.clear();

        for card in starter_deck.iter_shared() {
            self.deck.push(&card);
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
    pub fn get_burn_indices(&self, play_idx: i32) -> Array<i32> {
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
            burn_indices.push(i as i32);
            collected += 1;
        }

        burn_indices
    }

    #[func]
    pub fn play_card(&mut self, card_idx: i32, source_idx: i32, target_idx: i32) -> Error {
        // 1. 유효성 검사 (인덱스 범위)
        if card_idx as usize >= self.hand.len() {
            return Error::ERR_INVALID_PARAMETER;
        }

        let mut indices_to_remove = self.get_burn_indices(card_idx);

        // 사용할 카드 임시 확보 (아직 제거 안 함)
        let card_to_play = self.hand.at(card_idx as usize).clone();
        let card_bind = card_to_play.bind();

        if card_bind.cost as usize > indices_to_remove.len() {
            godot_print!(
                "Not enough cards to burn! Need {}, have {}.",
                card_bind.cost,
                indices_to_remove.len()
            );
            return Error::ERR_UNAVAILABLE;
        }

        let source_unit = match self.get_unit(source_idx) {
            Some(u) => u,
            None => return Error::ERR_INVALID_PARAMETER,
        };

        let mut target_unit = match self.get_unit(target_idx) {
            Some(u) => u,
            None => return Error::ERR_DOES_NOT_EXIST,
        };

        let dist = (target_idx - source_idx).abs();
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
