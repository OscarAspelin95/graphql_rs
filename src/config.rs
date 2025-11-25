pub struct Config {
    pub port: usize,
}

impl Config {
    pub fn new() -> Self {
        Self { port: 3000 }
    }
}
