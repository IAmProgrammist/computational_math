fn f(x: f64) -> f64 {
    2. * f64::exp(-0.2 * x) * f64::sin(3. * x) + f64::ln(1. + f64::powi(x, 2))
}

fn first(x: f64) -> f64 {
    let h = 0.01;
    return (f(x + h) - f(x)) / h;
}

fn second(x: f64) -> f64 {
    let h = 0.01;
    return (f(x + h) - f(x - h)) / (2. * h);
}

fn fourth(x: f64) -> f64 {
    let h = 0.01;
    return (f(x - 2. * h) - 8. * f(x - h) + 8. * f(x + h) - f(x + 2. * h)) / (12. * h);
}

fn differ2(x: f64) -> f64 {
    let h = 0.01;
    return (f(x - h) - 2. * f(x) + f(x + h)) / f64::powi(h, 2);
}

fn differ3(x: f64) -> f64 {
    let h = 0.01;
    return (f(x + 3.*h / 2.) - 3. * f(x + h / 2.) + 3. * f(x - h / 2.) - f(x - 3. * h / 2.)) / f64::powi(h, 3);
}

fn main() {
    let x = 0.;

    println!("Первая производная первого порядка: \n");
    let actual = 6.;
    println!("Абсолютная погрешность: {}", actual - first(x));
    println!("Относительная погрешность: {}\n\n", (actual - first(x)) / actual);

    println!("Первая производная второго порядка: \n");
    let actual = 6.;
    println!("Абсолютная погрешность: {}", actual - second(x));
    println!("Относительная погрешность: {}\n\n", (actual - second(x)) / actual);

    println!("Первая производная четвёртого порядка: \n");
    let actual = 6.;
    println!("Абсолютная погрешность: {}", actual - fourth(x));
    println!("Относительная погрешность: {}\n\n", (actual - fourth(x)) / actual);

    println!("Вторая производная: \n");
    let actual = -0.4;
    println!("Абсолютная погрешность: {}", actual - differ2(x));
    println!("Относительная погрешность: {}\n\n", (actual - differ2(x)) / actual);

    println!("Третья производная: \n");
    let actual = -53.28;
    println!("Абсолютная погрешность: {}", actual - differ3(x));
    println!("Относительная погрешность: {}\n\n", (actual - differ3(x)) / actual);
}