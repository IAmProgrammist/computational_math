pub fn makeComputation(principal: f32, rate: f32, periods: f32) -> f32 {
    principal * (1.0 + rate).powf(periods)
}

pub fn makeImprovedComputation(principal: f32, rate: f32, periods: f32) -> f32 {
    principal * (periods * (1.0 + rate).ln()).exp()
}