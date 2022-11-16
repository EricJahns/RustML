pub struct Datapoint {
    pub features: Vec<f32>,
    pub labels: Vec<i32>,
    pub positive_class: i32,
}

impl Datapoint {
    pub fn new(&self,features: Vec<f32>, label: Vec<i32>) -> Datapoint {
        Datapoint {
            features,
            label,
            label.iter().position(|&x| x == x).unwrap() as i32
        }
    }

    pub fn normalize(&mut self, min_features: Vec<f32>, max_features: Vec<f32>) -> None {
        for i in 0..self.features.len() {
            if (max_features[i] - min_features[i]) != 0.0 {
                self.features[i] = (self.features[i] - min_features[i]) / (max_features[i] - min_features[i]);
            }
            self.features[i] = -self.features[i];
        }
    }
}