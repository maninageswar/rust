use crate::game::combat::{CombatStats};

#[derive(Debug)]
pub enum EnemyType {
    Zombie(CombatStats),
    Skeleton(CombatStats),
}