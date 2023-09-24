use crate::util::sign_extend;

pub struct Memory {
    inner: Vec<u8>,
}

impl Memory {
    pub fn new(len: usize) -> Self {
        assert!(len % 4 == 0);

        Self {
            inner: vec![0; len],
        }
    }

    fn get(&self, i: usize) -> u8 {
        self.inner[i]
    }

    fn set(&mut self, i: usize, value: u8) {
        self.inner[i] = value;
    }

    // ---- Load ----
    pub fn load_byte(&self, i: usize) -> i32 {
        sign_extend(self.get(i) as u32, 8)
    }

    pub fn load_ubyte(&self, i: usize) -> i32 {
        self.get(i) as i32
    }

    pub fn load_halfword(&self, i: usize) -> i32 {
        sign_extend(
            u16::from_le_bytes([self.get(i), self.get(i + 1)]) as u32,
            16,
        )
    }

    pub fn load_uhalfword(&self, i: usize) -> i32 {
        u16::from_le_bytes([self.get(i), self.get(i + 1)]) as i32
    }

    pub fn load_word(&self, i: usize) -> i32 {
        i32::from_le_bytes([
            self.get(i),
            self.get(i + 1),
            self.get(i + 2),
            self.get(i + 3),
        ])
    }

    // Store
    pub fn store_byte(&mut self, i: usize, word: i32) {
        self.set(i, word as u8);
    }

    pub fn store_halfword(&mut self, i: usize, word: i32) {
        let b = (word as u16).to_le_bytes();
        self.set(i, b[0]);
        self.set(i + 1, b[1]);
    }

    pub fn store_word(&mut self, i: usize, word: i32) {
        let b = word.to_le_bytes();
        self.set(i, b[0]);
        self.set(i + 1, b[1]);
        self.set(i + 2, b[2]);
        self.set(i + 3, b[3]);
    }
}
