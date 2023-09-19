use std::{fmt, fmt::Formatter};

use crate::flags::MethodFlags;
use crate::method::descriptor::MethodDescriptor;
use crate::method::exception_table::ExceptionTable;
use crate::method::line_number_table::LineNumberTable;
use crate::utils::attribute::Attribute;
use crate::utils::instruction::Instruction;
use crate::utils::types::{BaseType, Type};

/// Models a method in a class
pub struct ClassFileMethod {
    pub flags: MethodFlags,
    pub name: String,
    /// 方法内部有着对应的简要的描述符,
    /// 例如 long method(int a) 的描述符为 (I)J
    /// 例如 void method(float a, int b) 的描述符为 (FI)V
    pub type_descriptor: String,
    /// 将描述符进行解析
    pub parsed_type_descriptor: MethodDescriptor,
    /// Generic attributes of the method
    // TODO: replace with some proper struct
    pub attributes: Vec<Attribute>,
    /// code 是每一个 method 都必须有的属性
    pub code: Option<ClassFileMethodCode>,
    pub deprecated: bool,
    /// exceptions 是每一个 method 都必须有的属性
    pub thrown_exceptions: Vec<String>,
}

impl fmt::Display for ClassFileMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {}: {}{} throws {:?}",
            self.flags,
            self.name,
            self.parsed_type_descriptor,
            if self.deprecated { " (deprecated)" } else { "" },
            self.thrown_exceptions,
        )?;
        if let Some(code) = &self.code {
            writeln!(f, "  code: {code}")?;
        }
        write!(f, "  raw_attributes: {:?}", self.attributes)
    }
}

impl ClassFileMethod {
    pub fn is_static(&self) -> bool {
        self.flags.contains(MethodFlags::STATIC)
    }

    pub fn is_native(&self) -> bool {
        self.flags.contains(MethodFlags::NATIVE)
    }

    pub fn is_void(&self) -> bool {
        self.parsed_type_descriptor.return_type.is_none()
    }

    pub fn returns(&self, expected_type: Type) -> bool {
        match self.parsed_type_descriptor.return_type {
            Some(Type::Base(BaseType::Int))
            | Some(Type::Base(BaseType::Short))
            | Some(Type::Base(BaseType::Char))
            | Some(Type::Base(BaseType::Byte))
            | Some(Type::Base(BaseType::Boolean)) => {
                Type::Base(BaseType::Int) == expected_type
            }
            _ => self.parsed_type_descriptor.return_type == Some(expected_type),
        }
    }
}

/// Code of a given method
#[derive(Debug, Default, PartialEq)]
pub struct ClassFileMethodCode {
    /// Maximum depth of the stack at any time
    pub max_stack: u16,
    /// Number of local variables used by the method
    pub max_locals: u16,
    /// Raw bytecode
    pub code: Vec<u8>,
    pub exception_table: ExceptionTable,
    pub line_number_table: Option<LineNumberTable>,

    /// Generic unmapped attributes of the code
    // TODO: replace with some proper struct
    pub attributes: Vec<Attribute>,
}

impl fmt::Display for ClassFileMethodCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "max_stack = {}, max_locals = {}, exception_table = {:?}, line_number_table: {:?}, attributes = {:?}, instructions:",
            self.max_stack, self.max_locals, self.exception_table, self.line_number_table, self.attributes,
        )?;

        let instructions = Instruction::parse_instructions(&self.code);
        if let Ok(instructions) = instructions {
            for (address, instruction) in instructions {
                writeln!(f, "    {address:3} {instruction:?}")?;
            }
        } else {
            writeln!(f, "    unparseable code: {:?}", self.code)?;
        }
        Ok(())
    }
}
