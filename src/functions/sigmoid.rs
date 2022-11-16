pub struct Sigmoid();

impl Sigmoid {
    pub fn evaluate(&self, x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }
    pub fn derivative(&self, x: f32) -> f32 {
        x * (1.0 - x)
    }
}
