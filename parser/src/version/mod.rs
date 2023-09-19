use std::fmt::Formatter;
use crate::error::{ClassFileParserError, ClassFileParserResult};

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, Default)]
pub struct ClassFileVersion {
    major_version: u16,
    minor_version: u16
}

impl ClassFileVersion {
    pub(crate) fn new(major_version: u16, minor_version: u16) -> ClassFileParserResult<Self> {
        let version = Self {
            major_version,
            minor_version,
        };
        return match ClassFileVersion::parse_to_sdk_version(&version) {
            Ok(v) => { Ok(version) }
            Err(_) => {
                Err(ClassFileParserError::ClassFileVersionError(major_version, minor_version))
            }
        }
    }
}

impl std::fmt::Display for ClassFileVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", ClassFileVersion::parse_to_sdk_version(&self))
    }
}

#[derive(Debug, PartialEq, Default, strum_macros::Display)]
pub enum SdkVersion {
    Jdk1_1,
    Jdk1_2,
    Jdk1_3,
    Jdk1_4,
    Jdk1_5,
    Jdk6,
    Jdk7,
    #[default]
    Jdk8,
    Jdk9,
    Jdk10,
    Jdk11,
    Jdk12,
    Jdk13,
    Jdk14,
    Jdk15,
    Jdk16,
    Jdk17,
    Jdk18,
    Jdk19,
    Jdk20,
    Jdk21,
    Jdk22,
}

impl ClassFileVersion {
    fn parse_to_sdk_version(value: &ClassFileVersion) -> ClassFileParserResult<SdkVersion> {
        match (value.major_version, value.minor_version) {
            (45, _) => Ok(SdkVersion::Jdk1_1),
            (46, _) => Ok(SdkVersion::Jdk1_2),
            (47, _) => Ok(SdkVersion::Jdk1_3),
            (48, _) => Ok(SdkVersion::Jdk1_4),
            (49, _) => Ok(SdkVersion::Jdk1_5),
            (50, _) => Ok(SdkVersion::Jdk6),
            (51, _) => Ok(SdkVersion::Jdk7),
            (52, _) => Ok(SdkVersion::Jdk8),
            (53, _) => Ok(SdkVersion::Jdk9),
            (54, _) => Ok(SdkVersion::Jdk10),
            (55, _) => Ok(SdkVersion::Jdk11),
            (56, _) => Ok(SdkVersion::Jdk12),
            (57, _) => Ok(SdkVersion::Jdk13),
            (58, _) => Ok(SdkVersion::Jdk14),
            (59, _) => Ok(SdkVersion::Jdk15),
            (60, _) => Ok(SdkVersion::Jdk16),
            (61, _) => Ok(SdkVersion::Jdk17),
            (62, _) => Ok(SdkVersion::Jdk18),
            (63, _) => Ok(SdkVersion::Jdk19),
            (64, _) => Ok(SdkVersion::Jdk20),
            (65, _) => Ok(SdkVersion::Jdk21),
            (66, _) => Ok(SdkVersion::Jdk22),
            _ => Err(ClassFileParserError::ClassFileVersionError(value.major_version, value.minor_version)),
        }
    }
}