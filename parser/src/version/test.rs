//! 关于 Java 版本的相关单元测试

use log::info;
use crate::error::ClassFileParserError::ClassFileVersionError;
use crate::log::{init_log, LogLevel};
use crate::version::{ClassFileVersion, SdkVersion};

/// 测试版本号与版本名称之间的转换
#[test]
fn test_transform() {
    init_log(LogLevel::INFO);

    let class_file_version = ClassFileVersion{
        major_version: 52,
        minor_version: 0,
    };
    assert_eq!(
        ClassFileVersion::parse_to_sdk_version(&class_file_version),
        Ok(SdkVersion::Jdk8)
    );


    let wrong_version = ClassFileVersion{
        major_version: 0,
        minor_version: 0,
    };
    let _ = ClassFileVersion::parse_to_sdk_version(&wrong_version).map_err(|e| {
        info!("Err: {}", e.to_string());
    });
    assert_eq!(
        ClassFileVersion::parse_to_sdk_version(&wrong_version),
        Err(ClassFileVersionError(0, 0))
    );
}