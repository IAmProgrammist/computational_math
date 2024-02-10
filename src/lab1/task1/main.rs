// демонстрация чувствительности результата вычисления к последовательности
// арифметических операций
fn main() {
    let num1: f32 = 0.23456789;
    let num2: f32 = 1.5678e+20;
    let num3: f32 = 1.2345e+10;
    let result1 = (num1 * num2) / num3;
    let result2 = (num1 / num3) * num2;
    let result3: f64 = num1 as f64 * num2 as f64 / num3 as f64;
    println!("({} * {}) / {} = {}", num1, num2, num3, result1);
    println!("({} / {}) * {} = {}", num1, num3, num2, result2);
    println!(" {} * {} / {} = {}", num1, num2, num3, result3);
}