use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Eq, Clone, Copy, Debug)]
#[godot(via = i32)] // Godot에서는 0, 1 정수로 취급
pub enum Faction {
    Player = 0,
    Enemy = 1,
}

impl Default for Faction {
    fn default() -> Self {
        Faction::Player
    }
}

#[derive(GodotClass, Debug)]
#[class(base=Node2D, init)]
pub struct Unit {
    #[export]
    pub hp: i32,
    #[export]
    pub max_hp: i32,
    #[export]
    action_point: i32,
    #[export]
    unit_name: GString,
    #[export]
    grid_index: i32,
    #[export]
    pub faction: Faction,

    base: Base<Node2D>,
}

#[godot_api]
impl Unit {
    #[func]
    pub fn setup(&mut self, name: GString, hp: i32, action_point: i32) {
        self.unit_name = name;
        self.hp = hp;
        self.max_hp = hp;
        self.action_point = action_point;
        self.base_mut()
            .emit_signal("hp_changed", &[hp.to_variant(), hp.to_variant()]);
    }

    #[func]
    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
        let hp = self.hp;
        let max_hp = self.max_hp;

        self.base_mut()
            .emit_signal("hp_changed", &[hp.to_variant(), max_hp.to_variant()]);
        if self.hp <= 0 {
            let idx = self.grid_index;
            self.base_mut().emit_signal("died", &[idx.to_variant()]);
        }
    }

    #[func]
    pub fn update_grid_index(&mut self, new_index: i32) {
        self.grid_index = new_index;
        self.base_mut()
            .emit_signal("grid_index_changed", &[new_index.to_variant()]);
    }

    #[func]
    pub fn update_faction(&mut self, faction: Faction) {
        self.faction = faction;
    }

    #[signal]
    fn grid_index_changed(new_index: i32);
    #[signal]
    fn hp_changed(hp: i32, max_hp: i32);
    #[signal]
    fn died(index: i32);
}
