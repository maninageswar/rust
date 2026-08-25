#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_shirt_count = 0;
        let mut blue_shirt_count = 0;
        for shirt_color in &self.shirts {
            match shirt_color {
                ShirtColor::Red => { red_shirt_count += 1 },
                ShirtColor::Blue => { blue_shirt_count += 1 },
            }
        }
        if red_shirt_count > blue_shirt_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let shirts_inventory1 = Inventory {
        shirts : vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user1_preference = Some(ShirtColor::Blue);
    println!("user1 has received {:#?} colored shirt", shirts_inventory1.giveaway(user1_preference));

    let user2_preference = None;
    println!("user2 has received {:#?} colored shirt", shirts_inventory1.giveaway(user2_preference));
}