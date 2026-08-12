use crate::game::combat::{CombatStats};

pub struct Player {
    pub name: String,
    pub combat_status: CombatStats,
}

impl Player {
    pub fn new(name: String, combat_status: CombatStats) -> Self {
        Self {
            name,
            combat_status
        }
    }

    pub fn is_alive(&self) -> bool {
        self.combat_status.is_character_alive()
    }
}