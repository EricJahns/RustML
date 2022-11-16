use datapoint::DataPoint;
use super::super::options::Options;

pub struct Dataset {
    TRAINING_SET_PERCENTAGE: f32,
    options: Options,
    pub data: Vec<DataPoint>,
}

impl Dataset {
    pub fn new(options: Options) -> Dataset {
        Dataset {
            TRAINING_SET_PERCENTAGE: 0.8,
            options,
            data: Vec::new(),
        }
    }

    pub fn add(&mut self, datapoint: DataPoint) {
        self.data.push(datapoint);
    }
    
}