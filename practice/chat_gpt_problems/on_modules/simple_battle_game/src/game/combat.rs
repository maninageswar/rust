use crate::game::{player::*, enemy::*};

#[derive(Debug)]
pub struct CombatStats {
    pub health: i32,
    pub attack: i32,
}

impl CombatStats {
    fn new(health: i32, attack: i32) -> Self {
        Self {
            health,
            attack
        }
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }
    
    fn is_character_alive(&self) -> bool {
        if self.health > 0 {
            true
        } else {
            false
        }
    }
}

fn player_attack(enemy: &mut EnemyType, amount: i32) {
    match enemy {
        EnemyType::Zombie(enemy_type) => enemy_type.take_damage(amount),
        EnemyType::Skeleton(enemy_type) => enemy_type.take_damage(amount)
    }
}