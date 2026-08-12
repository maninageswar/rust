use crate::game::combat::CombatStats;

#[derive(Debug)]
pub enum EnemyType {
    Zombie(CombatStats),
    Skeleton(CombatStats),
}

impl EnemyType {
    pub fn is_alive(&self) -> bool {
        match self {
            EnemyType::Zombie(combat_status) | EnemyType::Skeleton(combat_status) => {
                combat_status.is_character_alive()
            }
        }
    }
}
