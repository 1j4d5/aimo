#[derive(Default)]
pub struct CloseConfirmation {
    pub is_open: bool,
    pub buffer_idx: usize,
}

impl CloseConfirmation {
    pub fn ask(&mut self, idx: usize) {
        self.is_open = true;
        self.buffer_idx = idx;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}