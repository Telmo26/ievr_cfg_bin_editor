pub struct BinaryWriter<'a> {
    file: &'a mut [u8],
    position: usize,
}

impl<'a> BinaryWriter<'a> {
    pub fn new(file: &'a mut [u8]) -> BinaryWriter<'a> {
        BinaryWriter { file, position: 0 }
    }

    
}