use std::fmt;
use std::fmt::{Formatter, Write};
use std::str::Chars;
use itertools::Itertools;
use crate::error::ClassFileParserError::InvalidMethodTypeDescriptor;
use crate::error::ClassFileParserResult;
use crate::utils::types::Type;

/// Models the signature of a method, i.e. the type of the parameters it takes and the type
/// of the return value
/// 对方法签名进行建模, 可以从这里看出方法中最重要的其实就是参数和返回值
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MethodDescriptor {
    pub parameters: Vec<Type>,
    pub return_type: Option<Type>,
}

impl fmt::Display for MethodDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        f.write_str(&self.parameters.iter().join(", "))?;
        match &self.return_type {
            Some(field_type) => write!(f, ") -> {field_type}"),
            None => f.write_str(") -> void"),
        }
    }
}

impl MethodDescriptor {
    /// 解析传入的描述符, 需要解析为对应的输入输出类型。
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.3.3
    pub fn parse(descriptor: &str) -> ClassFileParserResult<MethodDescriptor> {
        let mut chars = descriptor.chars();
        match chars.next() {
            Some('(') => {
                let parameters = Self::parse_parameters(descriptor, &mut chars)?;
                if Some(')') == chars.next() {
                    let return_type = Self::parse_return_type(descriptor, &mut chars)?;
                    Ok(MethodDescriptor {
                        parameters,
                        return_type,
                    })
                } else {
                    Err(InvalidMethodTypeDescriptor(descriptor.to_string()))
                }
            }
            _ => Err(InvalidMethodTypeDescriptor(descriptor.to_string())),
        }
    }

    fn parse_parameters(
        descriptor: &str,
        chars: &mut Chars,
    ) -> ClassFileParserResult<Vec<Type>> {
        let mut parameters = Vec::new();
        loop {
            match chars.clone().next() {
                Some(')') => return Ok(parameters),
                Some(_) => {
                    let param = Type::parse_from(descriptor, chars)?;
                    parameters.push(param);
                }
                None => return Err(InvalidMethodTypeDescriptor(descriptor.to_string())),
            }
        }
    }

    fn parse_return_type(
        descriptor: &str,
        chars: &mut Chars,
    ) -> ClassFileParserResult<Option<Type>> {
        match chars.clone().next() {
            Some('V') => Ok(None),
            Some(_) => {
                let return_type = Some(Type::parse_from(descriptor, chars)?);
                if chars.next().is_none() {
                    Ok(return_type)
                } else {
                    Err(InvalidMethodTypeDescriptor(descriptor.to_string()))
                }
            }
            _ => Err(InvalidMethodTypeDescriptor(descriptor.to_string())),
        }
    }

    pub fn num_arguments(&self) -> usize {
        self.parameters.len()
    }
}