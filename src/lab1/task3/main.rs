fn main() {
    let pi = std::f32::consts::PI;
    let infinity = std::f32::INFINITY;
    let nan = std::f32::NAN;
    let smallest_positive = std::f32::MIN_POSITIVE;
    let largest_positive = std::f32::MAX;
    let smallest_negative = -std::f32::MIN_POSITIVE;
    println!("π: {}", float_to_binary_string(pi));
    println!("Infinity: {}", float_to_binary_string(infinity));
    println!("NaN: {}", float_to_binary_string(nan));
    println!(
        "Smallest Positive Number: {}",
        float_to_binary_string(smallest_positive)
    );
    println!(
        "Largest Positive Number: {}",
        float_to_binary_string(largest_positive)
    );
    println!(
        "Smallest Negative Number: {}",
        float_to_binary_string(smallest_negative)
    );
}
fn float_to_binary_string(num: f32) -> String {
    let bits = num.to_bits();
    format!("{:032b}", bits)
}
