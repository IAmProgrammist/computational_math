fn f(x: f64) -> f64 {
    (f64::exp(0.2 * x) * f64::tanh(1.5 * x)) / (1.2 + f64::cos(x))
}

fn first_l(mut a: f64, b: f64) -> f64 {
    let h = 0.01;
    let n = ((b - a) / h) as i64;
    let mut res = 0.0;

    for _i in 0..n {
        res += f(a) * h;

        a += h;
    };

    res
}

fn first_r(mut a: f64, b: f64) -> f64 {
    let h = 0.01;
    let n = ((b - a) / h) as i64;
    a += h;
    let mut res = 0.0;

    for _i in 0..n {
        res += f(a) * h;

        a += h;
    };

    res
}

fn second_mid_sq(a: f64, b: f64) -> f64 {
    let h = 0.01;
    let n = ((b - a) / h) as i64;
    let mut res = 0.0;
    let mut xim1 = a;
    let mut xi = a + h;

    for _i in 1..n {
        res += f((xim1 + xi) / 2.) * h;

        xim1 += h;
        xi += h;
    };

    res
}

fn second_trap(a: f64, b: f64) -> f64 {
    let h = 0.01;
    let n = ((b - a) / h) as i64;
    let mut res = 0.0;
    let mut xim1 = a;
    let mut xi = a + h;

    for _i in 1..n {
        res += (f(xim1) + f(xi)) * h;

        xim1 += h;
        xi += h;
    };

    res / 2.
}

fn fourth_simp(a: f64, b: f64) -> f64 {
    let h = 0.01;
    let n = ((b - a) / h) as i64;
    let mut res = f(a) + f(b);

    for i in 1..n {
        if i % 2 != 0 {
            res += 4. * f(a + i as f64 * h);
        } else {
            res += 2. * f(a + i as f64 * h);
        }
    };

    res * h / 3.
}

fn main() {
    let a = 0.0;
    let b = 10.0;

    let actual = 60.797393564866084;

    let ans_first_l = first_l(a, b);
    println!("Метод левых прямоугольников: {}", ans_first_l);
    println!("Абсолютная погрешность: {}", actual - ans_first_l);
    println!("Относительная погрешность: {}\n\n", (actual - ans_first_l) / actual);

    let ans_first_r = first_r(a, b);
    println!("Метод правых прямоугольников: {}", ans_first_r);
    println!("Абсолютная погрешность: {}", actual - ans_first_r);
    println!("Относительная погрешность: {}\n\n", (actual - ans_first_r) / actual);

    let ans_second_mid_sq = second_mid_sq(a, b);
    println!("Метод средних прямоугольников: {}", ans_second_mid_sq);
    println!("Абсолютная погрешность: {}", actual - ans_second_mid_sq);
    println!("Относительная погрешность: {}\n\n", (actual - ans_second_mid_sq) / actual);

    let ans_second_trap = second_trap(a, b);
    println!("Метод трапеций: {}", ans_second_trap);
    println!("Абсолютная погрешность: {}", actual - ans_second_trap);
    println!("Относительная погрешность: {}\n\n", (actual - ans_second_trap) / actual);

    let ans_fourth_simp = fourth_simp(a, b);
    println!("Метод Симпсона: {}", ans_fourth_simp);
    println!("Абсолютная погрешность: {}", actual - ans_fourth_simp);
    println!("Относительная погрешность: {}\n\n", (actual - ans_fourth_simp) / actual);
}