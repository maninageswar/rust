// see the entire problem statement given by chatgpt at: https://chatgpt.com/c/6a75a073-518c-83ee-b6c4-9036707c905a

mod game;

use game::{combat::*, enemy::*, player::*};

fn main() {
    let mut player1: Player = Player::new(String::from("nathan"), CombatStats::new(100, 25));

    let mut zombie: EnemyType = EnemyType::Zombie(CombatStats::new(50, 10));
    let mut skeleton: EnemyType = EnemyType::Skeleton(CombatStats::new(40, 15));

    // 1. Player attacks Zombie
    player_attack(&player1, &mut zombie);

    // 2. Zombie attacks Player
    enemy_attack(&zombie, &mut player1);

    // 3. Player attacks Zombie
    player_attack(&player1, &mut zombie);

    // 4. Player attacks Skeleton
    player_attack(&player1, &mut skeleton);

    // 5. Skeleton attacks Player
    enemy_attack(&skeleton, &mut player1);

    // 6. Check whether Player is alive
    if player1.is_alive() == true {
        println!("the player1 is alive");
    } else {
        println!("the player1 is dead");
    }

    // 7. Check whether Zombie is alive
    if zombie.is_alive() == true {
        println!("the zombie is alive");
    } else {
        println!("the zombie is dead");
    }

    // 8. Check whether Skeleton is alive
    if skeleton.is_alive() == true {
        println!("the skeleton is alive");
    } else {
        println!("the skeleton is dead");
    }
}
