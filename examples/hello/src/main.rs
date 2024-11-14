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
    println!("{}", msg)
}
