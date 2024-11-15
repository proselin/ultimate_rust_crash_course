use hello::{english, russian, spanish};

fn main() {
    english::greet();
    spanish::greet();
    russian::greet();
    mainada();
}

fn mainada() {
    // let mut num = 5;
    // let mut msg = "";
    // if num == 5 {
    //     msg = "five";
    // }else if num == 6 {
    //     msg = "six";
    // }
    let num = 6;
    let msg = if num == 5 {
        "fire"
    }else if num == 6 {
        "six"
    } else {
        "seven"
    };
    println!("{}", msg);

    let fox = RedFox::new();
    print!("{} and {}", fox.enemy, fox.life);
    let life_left = fox.life;
    fox.enemy = life_left;
    fox.life = life_left;
    
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70
        }
    }
}

struct RedFox { 
    enemy: bool,
    life: u8
}
