// E = (x1^2 + e^x2 - 7) ^ 2 + (atan(x1) + cos(x2)) ^ 2

// E' (x1) = 4 * x1 * (x1^2 + e^x2 - 7) + 2 * (1 / (1 + x1^2)) * (atan(x1) + cos(x2)) 
// E' (x2) = 2 * e^x2 * (x1^2 + e^x2 - 7) + 2 * (-sin(x2)) * (atan(x1) + cos(x2)) 

fn get_residual(x1: f64, x2: f64) -> f64 {
    (x1.powi(2) + f64::exp(x2) - 7.).powi(2) + (f64::atan(x1) + f64::cos(x2)).powi(2)
}

fn get_residual_x1(x1: f64, x2: f64) -> f64 {
    4. * x1 * (x1.powi(2) + f64::exp(x2) - 7.) + 2. * (1. / (1. + x1.powi(2))) * (f64::atan(x1) + f64::cos(x2)) 
} 

fn get_residual_x2(x1: f64, x2: f64) -> f64 {
    2. * f64::exp(x2) * (x1.powi(2) + f64::exp(x2) - 7.) + 2. * (-f64::sin(x2)) * (f64::atan(x1) + f64::cos(x2)) 
} 

fn main() {
    let learning_speed = 0.000001;
    let epsilon = 0.000000001;

    let mut x1 = 0.;
    let mut x2 = 0.;

    while get_residual(x1, x2) > epsilon {
        (x1, x2) = (x1 - learning_speed * get_residual_x1(x1, x2), x2 - learning_speed * get_residual_x2(x1, x2));
    }

    println!("Ответ: x_1: {x1}; x_2: {x2}");
}