pub struct DFF {
    pub prev_val: bool,
}

impl DFF {
    pub fn new() -> DFF {
        DFF { prev_val: false }
    }

    pub fn out(&self) -> bool {
        self.prev_val
    }

    pub fn clock(&mut self, input: bool) {
        self.prev_val = input;
    }
}
