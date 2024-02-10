// демонстрация накопления погрешности для итерационного процесса
// версия для одинарной точности
fn iter(numbers: &[f32], iterations: i32) {
    println!("Итерационный метод: ");
    for &number in numbers {
        let mut result = number;
        for _ in 0..iterations {
            result = result.sqrt(); // послед. извлечение квадратного корня
        }
        for _ in 0..iterations {
            result = result * result; // послед. возведение числа в квадрат
        }
        let error = (number - result).abs();
        println!(
            "Исх-е значение: {:e}, результат: {:e}, абс-ая погрешность:
    {:e}, отн-ая погрешность: {:e} (%)",
            number,
            result,
            error,
            error * 100. / number
        );
    }
}

// замена итерации функцией
// версия для одинарной точности c powf
fn non_iter(numbers: &[f32], iterations: i32) {
    println!("Безытерационный метод: ");
    for &number in numbers {
        // извлекаем корень
        let intermediate = number.powf(1.0f32 / (1 << iterations) as f32);
        // восстанавливаем значение
        let result = intermediate.powf((1 << iterations) as f32);
        let error = (number - result).abs();
        println!(
            "Исх-е значение: {:e}, результат: {:e}, абс-ая погрешность:
    {:e}, отн-ая погрешность: {:e} (%)",
            number,
            result,
            error,
            error * 100. / number
        );
    }
}

fn main() {
    let numbers = [
        1.0f32,
        20.,
        300.,
        4000.,
        5e6,
        f32::MIN_POSITIVE,
        f32::MAX * 0.99,
    ]; // вектор с числами одинарной точности

    let iterations = 10; // число итераций
    
    iter(&numbers, iterations);
    non_iter(&numbers, iterations)
}