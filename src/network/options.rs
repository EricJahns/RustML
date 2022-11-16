use super::super::functions::sigmoid::Sigmoid;

pub struct Options {
    pub file_name : String,
    pub hidden_layers : Vec<i32>,
    pub learning_rate : f32,
    pub epoch_limit: u32,
    pub batch_size: u32,
    pub lambda: f32,
    pub shuffle_data: bool,
    pub verbosity: u8,
    pub current_example: u32,
    pub function: Sigmoid,
}

impl Options {
    pub fn new() -> Options {
        Options {
            file_name : String::from(""),
            hidden_layers : vec![],
            learning_rate : 0.01,
            epoch_limit: 1000,
            batch_size: 1,
            lambda: 0.0,
            shuffle_data: false,
            verbosity: 1,
            current_example: 1,
            function: Sigmoid(),
        }
    }

    pub fn read_input(&mut self, input: Vec<String>) {
        for mut i in 0..input.len() {
            match input[i].as_ref() {
                "-f" => {
                    self.file_name = input[i+1].clone();
                },
                "-h" => {
                    i = i + 1;
                    let mut hidden_layers = vec![input[i].parse::<i32>().unwrap()];
                    for j in 1..hidden_layers.len() {
                        hidden_layers.push(input[i+j].parse::<i32>().unwrap());
                    }
                    self.hidden_layers = hidden_layers;
                },
                "-a" => {
                    self.learning_rate = input[i+1].parse::<f32>().unwrap();
                },
                "-e" => {
                    self.epoch_limit = input[i+1].parse::<u32>().unwrap();
                },
                "-m" => {
                    self.batch_size = input[i+1].parse::<u32>().unwrap();
                },
                "-l" => {
                    self.lambda = input[i+1].parse::<f32>().unwrap();
                },
                "-r" => {
                    self.shuffle_data = true;
                },
                "-v" => {
                    self.verbosity = input[i+1].parse::<u8>().unwrap();
                }
                _ => {
                    println!("Invalid input: {}", input[i]);
                }
            }
        }
    }
}