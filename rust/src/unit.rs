use godot::prelude::*;

use crate::data::{GridIdx, UID};

#[derive(GodotConvert, Var, Export, PartialEq, Eq, Clone, Copy, Debug)]
#[godot(via = i32)]
pub enum Faction {
    Player = 0,
    Enemy = 1,
}

impl Default for Faction {
    fn default() -> Self {
        Faction::Player
    }
}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct UnitRes {
    #[export]
    pub name: GString,
    #[export]
    pub hp: i32,
    #[export]
    pub action_point: i32,

    base: Base<Resource>,
}

#[derive(GodotClass, Debug)]
#[class(base=Node2D, init)]
pub struct Unit {
    #[export]
    pub hp: i64,
    #[export]
    pub max_hp: i64,
    #[export]
    action_point: i64,
    #[export]
    unit_name: GString,
    #[export]
    pub faction: Faction,

    uid: UID,
    pub grid_index: GridIdx,

    base: Base<Node2D>,
}

#[godot_api]
impl Unit {
    pub fn setup(&mut self, grid_index: GridIdx, uid: UID) {
        self.grid_index = grid_index;
        self.uid = uid;
        let hp = self.hp;
        self.base_mut()
            .emit_signal("hp_changed", &[hp.to_variant(), hp.to_variant()]);
    }

    #[func]
    pub fn take_damage(&mut self, amount: i64) {
        self.hp -= amount;
        let hp = self.hp;
        let max_hp = self.max_hp;

        self.base_mut()
            .emit_signal("hp_changed", &[hp.to_variant(), max_hp.to_variant()]);
        if self.hp <= 0 {
            let uid = self.uid.get();
            self.base_mut().emit_signal("died", &[uid.to_variant()]);
        }
    }

    #[func]
    pub fn update_faction(&mut self, faction: Faction) {
        self.faction = faction;
    }

    #[func]
    pub fn get_uid(&self) -> UID {
        self.uid
    }

    #[func]
    pub fn get_grid_index(&self) -> GridIdx {
        self.grid_index
    }

    #[signal]
    fn grid_index_changed(new_index: i64);
    #[signal]
    fn hp_changed(hp: i64, max_hp: i64);
    #[signal]
    fn died(index: i64);
}
