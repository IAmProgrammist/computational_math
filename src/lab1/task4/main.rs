extern crate algr;

use algr::lab1::task4::{make_computation, make_improved_computation};

fn main() {
    let pri = 1.0f32;
    let rat = 132.3112;
    let per = 3.0;

    println!("{:.32}", make_computation(pri, rat, per));
    println!("{:.32}", make_improved_computation(pri, rat, per));
    println!("{:.32}", make_computation(pri, rat, per) - make_improved_computation(pri, rat, per));
}