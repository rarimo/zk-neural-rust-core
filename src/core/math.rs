pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_folded(logits: Vec<f32>) -> f32 {
    logits.into_iter().map(sigmoid).fold(0.0_f32, f32::max)
}
