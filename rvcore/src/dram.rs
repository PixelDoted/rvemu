#[derive(Debug)]
pub struct DRam {
    inner: Vec<u8>,
}

impl DRam {
    pub fn new(len: usize) -> Self {
        assert!(len % 4 == 0);

        Self {
            inner: vec![0; len],
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, i: usize) -> u8 {
        self.inner[i]
    }

    fn set(&mut self, i: usize, value: u8) {
        self.inner[i] = value;
    }

    // ---- Load ----
    pub fn load(&self, i: usize, size: u8) -> u32 {
        match size {
            8 => self.get(i) as u32,
            16 => u16::from_le_bytes([self.get(i), self.get(i + 1)]) as u32,
            32 => u32::from_le_bytes([
                self.get(i),
                self.get(i + 1),
                self.get(i + 2),
                self.get(i + 3),
            ]),
            _ => unimplemented!(),
        }
    }

    // Store
    pub fn store(&mut self, i: usize, size: u8, value: u32) {
        match size {
            8 => self.set(i, value as u8),
            16 => {
                let b = (value as u16).to_le_bytes();
                self.set(i, b[0]);
                self.set(i + 1, b[1]);
            }
            32 => {
                let b = value.to_le_bytes();
                self.set(i, b[0]);
                self.set(i + 1, b[1]);
                self.set(i + 2, b[2]);
                self.set(i + 3, b[3]);
            }
            _ => unimplemented!(),
        }
    }
}
