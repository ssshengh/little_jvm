use std::{fmt, fmt::Formatter};

use crate::flags::FieldFlags;
use crate::utils::types::Type;

/// 对类中的字段进行建模
pub struct ClassFileField {
    pub flags: FieldFlags,
    pub name: String,
    pub type_descriptor: Type,
    /// final 修饰的字段，将会有一个 attribute 进行修饰
    pub constant_value: Option<FieldConstantValue>,
    pub deprecated: bool,
}

impl fmt::Display for ClassFileField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} = {} [constant {:?}] {}",
            self.flags,
            self.name,
            self.type_descriptor,
            self.constant_value,
            if self.deprecated { "(deprecated)" } else { "" }
        )
    }
}

/// Possible constant values of a field
#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum FieldConstantValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}