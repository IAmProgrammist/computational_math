extern crate algr;

use algr::lab1::task4::{makeComputation, makeImprovedComputation};

fn main() {
    let temp: f32 = 2.25;
    let temp2: f32 = 0.5;
    let oh: f32 = 1000.0;

    let pri = 1.0f32;
    let rat = temp.powf(oh) - 1.0;
    let per = temp2.powf(oh);

    println!("{:.32}", makeComputation(pri, rat, per));
    println!("{:.32}", makeImprovedComputation(pri, rat, per));
    println!("{:.32}", makeComputation(pri, rat, per) - makeImprovedComputation(pri, rat, per));
}