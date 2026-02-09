#[derive(Debug, Default)]
pub struct BinaryWriter {
    data: Vec<u8>,
    position: usize,
}

impl BinaryWriter {
    pub fn set_position(&mut self, position: usize) {
        if position > self.data.len() {
            self.data.resize(position, 0);
        }

        self.position = position;
    }    

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let end_pos = self.position + bytes.len();
        
        // If we are writing past the current end, grow the vector with zeros
        if end_pos > self.data.len() {
            self.data.resize(end_pos, 0);
        }

        // Copy bytes into place
        self.data[self.position..end_pos].copy_from_slice(bytes);
        self.position = end_pos;
    }

    pub fn write_u8(&mut self, data: u8) {
        self.write_bytes(&[data]);
    }

    pub fn write_i16(&mut self, data: i16) {
        self.write_bytes(&data.to_le_bytes());
    }

    pub fn write_u16(&mut self, data: u16) {
        self.write_bytes(&data.to_le_bytes());
    }

    pub fn write_i32(&mut self, data: i32) {
        self.write_bytes(&data.to_le_bytes());
    }

    pub fn write_u32(&mut self, data: u32) {
        self.write_bytes(&data.to_le_bytes());
    }

    pub fn write_i64(&mut self, data: i64) {
        self.write_bytes(&data.to_le_bytes());
    }

    pub fn write_f32(&mut self, data: f32) {
        self.write_bytes(&data.to_le_bytes());
    }

    pub fn write_f64(&mut self, data: f64) {
        self.write_bytes(&data.to_le_bytes());
    }

    /// Writes a string. 
    /// NOTE: This writes raw bytes. For T2b/Games, you usually want 
    /// `write_string_null_terminated` or to handle encoding externally.
    pub fn write_string(&mut self, data: &str, null_terminated: bool) {
        self.write_bytes(data.as_bytes());
        if null_terminated { self.write_u8(0); }
    }

    pub fn write_alignment(&mut self, align: usize, padding: u8) {
        let remainder = self.position % align;
        if remainder != 0 {
            let pad_len = align - remainder;
            let padding_bytes = vec![padding; pad_len];
            self.write_bytes(&padding_bytes);
        }
    }
}