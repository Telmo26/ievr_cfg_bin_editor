use memmap2::Mmap;

pub struct BinaryReader<'a> {
    file: &'a Mmap,
    position: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(file: &Mmap) -> BinaryReader {
        BinaryReader { file, position: 0 }
    }

    pub fn read_u32(&mut self) -> u32 {
        let v = u32::from_le_bytes(self.file[self.position..self.position + 4].try_into().unwrap());
        self.position += 4;
        v
    }

    pub fn read_i64(&mut self) -> i64 {
        let v = i64::from_le_bytes(self.file[self.position..self.position + 8].try_into().unwrap());
        self.position += 8;
        v
    }

    pub fn read_i32(&mut self) -> i32 {
        let v = i32::from_le_bytes(self.file[self.position..self.position + 4].try_into().unwrap());
        self.position += 4;
        v
    }

    pub fn read_i16(&mut self) -> i16 {
        let v = i16::from_le_bytes(self.file[self.position..self.position + 2].try_into().unwrap());
        self.position += 2;
        v
    }

    pub fn read_f32(&mut self) -> f32 {
        let v = f32::from_le_bytes(self.file[self.position..self.position + 4].try_into().unwrap());
        self.position += 4;
        v
    }

    pub fn read_bool(&mut self) -> bool {
        let v = self.read_i32();
        v != 0
    }

    pub fn read_bytes(&mut self, count: usize) -> &[u8] {
        let v = &self.file[self.position..self.position + count];
        self.position += count;
        v
    }

    pub fn read_byte(&mut self) -> u8 {
        let v = self.file[self.position];
        self.position += 1;
        v
    }

    pub fn skip(&mut self, delta: usize) {
        self.position += delta;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn seek_alignment(&mut self, align: usize) {
        while self.position % align != 0 {
            self.position += 1;
        }
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn file_size(&self) -> usize {
        self.file.len()
    }
}

