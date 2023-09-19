use cesu8::from_java_cesu8;
use crate::error::{ClassFileParserError, ClassFileParserResult};

/// A buffer reader, used to marshall data from a generic byte array
pub struct Buffer<'a> {
    buffer: &'a [u8],
    position: usize,
}


impl<'a> Buffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Buffer {
            buffer: data,
            position: 0,
        }
    }

    /// 移动位置指针到 size 字节之后, 并读取 size 字节的数据
    fn advance(&mut self, size: usize) -> ClassFileParserResult<&'a [u8]> {
        if self.position + size > self.buffer.len() {
            Err(ClassFileParserError::UnexpectedEndOfData)
        } else {
            let slice = &self.buffer[self.position..self.position + size];
            self.position += size;
            Ok(slice)
        }
    }

    pub fn read_u8(&mut self) -> ClassFileParserResult<u8> {
        self.advance(std::mem::size_of::<u8>())
            .map(|bytes| u8::from_be_bytes(bytes.try_into().expect("Read bytes error: u8. UB, please check.")))
    }

    pub fn read_u16(&mut self) -> ClassFileParserResult<u16> {
        self.advance(std::mem::size_of::<u16>())
            .map(|bytes| u16::from_be_bytes(bytes.try_into().expect("Read bytes error: u16. UB, please check.")))
    }

    pub fn read_u32(&mut self) -> ClassFileParserResult<u32> {
        self.advance(std::mem::size_of::<u32>())
            .map(|bytes| u32::from_be_bytes(bytes.try_into().expect("Read bytes error: u32. UB, please check.")))
    }

    pub fn read_i32(&mut self) -> ClassFileParserResult<i32> {
        self.advance(std::mem::size_of::<i32>())
            .map(|bytes| i32::from_be_bytes(bytes.try_into().expect("Read bytes error: i32. UB, please check.")))
    }

    pub fn read_i64(&mut self) -> ClassFileParserResult<i64> {
        self.advance(std::mem::size_of::<i64>())
            .map(|bytes| i64::from_be_bytes(bytes.try_into().expect("Read bytes error: i64. UB, please check.")))
    }

    pub fn read_f32(&mut self) -> ClassFileParserResult<f32> {
        self.advance(std::mem::size_of::<f32>())
            .map(|bytes| f32::from_be_bytes(bytes.try_into().expect("Read bytes error: f32. UB, please check.")))
    }

    pub fn read_f64(&mut self) -> ClassFileParserResult<f64> {
        self.advance(std::mem::size_of::<f64>())
            .map(|bytes| f64::from_be_bytes(bytes.try_into().expect("Read bytes error: f64. UB, please check.")))
    }

    pub fn read_utf8(&mut self, len: usize) -> ClassFileParserResult<String> {
        self.advance(len)
            .and_then(|bytes| from_java_cesu8(bytes).map_err(|_| ClassFileParserError::InvalidCesu8String))
            .map(|cow_string| cow_string.into_owned())
    }

    pub fn read_bytes(&mut self, len: usize) -> ClassFileParserResult<&'a [u8]> {
        self.advance(len)
    }

    #[allow(dead_code)]
    pub fn has_more_data(&self) -> bool {
        self.position < self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::buffer::Buffer;

    #[test]
    fn buffer_works() {
        let data = vec![0x00, 0x00, 0x00, 0x42];
        let mut buffer = Buffer::new(&data);

        assert!(buffer.has_more_data());
        assert_eq!(0x42u32, buffer.read_u32().expect("Read bytes error: . UB, please check."));
        assert!(!buffer.has_more_data());

        assert!(buffer.read_u32().is_err());
    }
}