fn get_fx(x: f64) -> f64 {
    f64::ln(7. - f64::tan(-f64::cos(x)).powi(2))
}

fn main() {
    let epsilon = 0.000000001;
    let mut prev = -1.;
    let x_2 = loop {
        let new_x = get_fx(prev);

        if f64::abs(new_x - prev) < epsilon {
            break new_x; 
        }

        prev = new_x;
    };

    let x_1 = f64::tan(-f64::cos(x_2));

    println!("Ответ: x_1: {x_1}; x_2: {x_2}");
}