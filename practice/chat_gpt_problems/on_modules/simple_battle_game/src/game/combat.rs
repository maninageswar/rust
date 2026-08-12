use crate::game::{enemy::*, player::*};

#[derive(Debug)]
pub struct CombatStats {
    pub health: i32,
    pub attack: u32,
}

impl CombatStats {
    pub fn new(health: i32, attack: u32) -> Self {
        Self { health, attack }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health -= damage as i32;
    }

    pub fn is_character_alive(&self) -> bool {
        if self.health > 0 { true } else { false }
    }
}

pub fn player_attack(player: &Player, enemy: &mut EnemyType) {
    match enemy {
        EnemyType::Zombie(combat_status) => {
            println!(
                "Player attacks Zombie for {} damage",
                player.combat_status.attack
            );
            combat_status.take_damage(player.combat_status.attack);
        }
        EnemyType::Skeleton(combat_status) => {
            println!(
                "Player attacks Zombie for {} damage",
                player.combat_status.attack
            );
            combat_status.take_damage(player.combat_status.attack);
        }
    }
}

pub fn enemy_attack(enemy: &EnemyType, player: &mut Player) {
    let enemy_attack: u32 = match enemy {
        EnemyType::Zombie(combat_status) => {
            println!("Zombie attacks Player for {} damage", combat_status.attack);
            combat_status.attack
        }
        EnemyType::Skeleton(combat_status) => {
            println!("Zombie attacks Player for {} damage", combat_status.attack);
            combat_status.attack
        }
    };
    player.combat_status.take_damage(enemy_attack);
}
