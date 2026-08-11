use crate::game::combat::{CombatStats};

pub struct Player {
    pub name: String,
    pub combat_status: CombatStats,
}

impl Player {
    fn new(name: String, combat_status: CombatStats) -> Self {
        Self {
            name,
            combat_status
        }
    }
}