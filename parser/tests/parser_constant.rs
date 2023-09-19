#[cfg(test)]
mod test {
    use parser::{class_parser, read_buffer};
    use parser::log::{init_log, LogLevel};

    #[test]
    fn test_read_file() {
        init_log(LogLevel::INFO);

        // 将 Class 文件识别为 bytes
        let bytes = include_bytes!("./classes/Constants.class");
        let class_file = read_buffer(bytes).unwrap();

        println!("The readed class file is: \n {}",  class_file);
    }
}