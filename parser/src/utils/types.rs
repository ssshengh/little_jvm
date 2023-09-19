use std::fmt;
use std::fmt::Formatter;
use std::str::Chars;
use itertools::Itertools;
use crate::error::ClassFileParserError::InvalidFiledTypeDescriptor;
use crate::error::ClassFileParserResult;

/// 对字段进行建模, 注意字段也可以是方法的一个参数
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Primitive types
    Base(BaseType),

    /// Standard object
    Object(String),

    /// Array
    Array(Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Base(base) => write!(f, "{base}"),
            Type::Object(class) => f.write_str(class),
            Type::Array(component_type) => write!(f, "{component_type}[]"),
        }
    }
}

/// Possible primitive types
#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
#[repr(u8)]
pub enum BaseType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
}

impl Type {
    /// Parses a type descriptor as specified in the JVM specs:
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.3.2
    pub fn parse(type_descriptor: &str) -> ClassFileParserResult<Type> {
        let mut chars = type_descriptor.chars();
        let descriptor = Self::parse_from(type_descriptor, &mut chars)?;
        match chars.next() {
            None => Ok(descriptor),
            Some(_) => Err(InvalidFiledTypeDescriptor(type_descriptor.to_string())),
        }
    }

    /// 从描述符中解析字段类型, 因为在字段的描述符中, 使用了特定的约束来表述类型
    pub(crate) fn parse_from(
        type_descriptor: &str,
        chars: &mut Chars,
    ) -> ClassFileParserResult<Type> {
        let first_char = chars
            .next()
            .ok_or(InvalidFiledTypeDescriptor(type_descriptor.to_string()))?;

        Ok(match first_char {
            'B' => Type::Base(BaseType::Byte),
            'C' => Type::Base(BaseType::Char),
            'D' => Type::Base(BaseType::Double),
            'F' => Type::Base(BaseType::Float),
            'I' => Type::Base(BaseType::Int),
            'J' => Type::Base(BaseType::Long),
            'S' => Type::Base(BaseType::Short),
            'Z' => Type::Base(BaseType::Boolean),
            'L' => {
                let class_name: String = chars.take_while_ref(|c| *c != ';').collect();
                match chars.next() {
                    Some(';') => Type::Object(class_name),
                    _ => return Err(InvalidFiledTypeDescriptor(type_descriptor.to_string())),
                }
            }
            '[' => {
                let component_type = Self::parse_from(type_descriptor, chars)?;
                Type::Array(Box::new(component_type))
            }
            _ => return Err(InvalidFiledTypeDescriptor(type_descriptor.to_string())),
        })
    }
}