pub fn make_computation(principal: f32, rate: f32, periods: f32) -> f32 {
    principal * (1.0 + rate).powf(periods)
}

pub fn make_improved_computation(principal: f32, rate: f32, periods: f32) -> f32 {
    principal * (periods * (1.0 + rate).ln()).exp()
}