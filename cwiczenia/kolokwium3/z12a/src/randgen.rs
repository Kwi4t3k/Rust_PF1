pub struct RandGen {
    seed: i64,
}

impl RandGen {
    pub fn new(seed: i64) -> Self {
        Self { seed }
    }

    pub fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64 {
        self.seed = (self.seed * 134775813 + 1) % 4294967296;
        self.seed % (max_rand - min_rand + 1) + min_rand
    }
}