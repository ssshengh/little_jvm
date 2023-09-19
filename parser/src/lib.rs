use crate::class::ClassFile;
use crate::class_parser::ClassFileReader;
use crate::error::ClassFileParserResult;

mod version;
mod error;
pub mod log;
mod constant_pool;
mod flags;
mod field;
mod method;
mod utils;
pub mod class;
pub mod class_parser;

/// 将数据读取为一个 Class 文件的抽象
pub fn read_buffer(buf: &[u8]) -> ClassFileParserResult<ClassFile>{
    ClassFileReader::new(buf).read()
}