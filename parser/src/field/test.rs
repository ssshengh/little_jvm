use crate::error::ClassFileParserError;
use crate::field::class_filed::{ClassFileField};
use crate::flags::FieldFlags;
use crate::utils::types::{BaseType, Type};

#[test]
fn test_filed() {
    let object_filed_type = Type::Object("StringNum".to_string());
    let base_filed_type = Type::Base(BaseType::Boolean);

    let flag = FieldFlags::PUBLIC | FieldFlags::STATIC;
    let filed = ClassFileField {
        flags: flag,
        name: "obj1".to_string(),
        type_descriptor: object_filed_type,
        constant_value: None,
        deprecated: true,
    };
    println!("{}", filed);
}

#[test]
fn cannot_parse_empty_descriptor() {
    assert!(matches!(
            Type::parse(""),
            Err(ClassFileParserError::InvalidFiledTypeDescriptor(s)) if s.is_empty()
        ));
}

#[test]
fn cannot_parse_invalid_primitive() {
    assert!(matches!(
            Type::parse("W"),
            Err(ClassFileParserError::InvalidFiledTypeDescriptor(s)) if s == "W"
        ));
}

#[test]
fn cannot_parse_missing_semicolon() {
    assert!(matches!(
            Type::parse("Ljava/lang/String"),
            Err(ClassFileParserError::InvalidFiledTypeDescriptor(s)) if s == "Ljava/lang/String"
        ));
}

#[test]
fn cannot_parse_invalid_array() {
    assert!(matches!(
            Type::parse("["),
            Err(ClassFileParserError::InvalidFiledTypeDescriptor(s)) if s == "["
        ));
}

#[test]
fn can_parse_primitive_descriptors() {
    assert_eq!(Ok(Type::Base(BaseType::Byte)), Type::parse("B"));
    assert_eq!(Ok(Type::Base(BaseType::Char)), Type::parse("C"));
    assert_eq!(Ok(Type::Base(BaseType::Double)), Type::parse("D"));
    assert_eq!(Ok(Type::Base(BaseType::Float)), Type::parse("F"));
    assert_eq!(Ok(Type::Base(BaseType::Int)), Type::parse("I"));
    assert_eq!(Ok(Type::Base(BaseType::Long)), Type::parse("J"));
    assert_eq!(Ok(Type::Base(BaseType::Short)), Type::parse("S"));
    assert_eq!(
        Ok(Type::Base(BaseType::Boolean)),
        Type::parse("Z")
    );
}

#[test]
fn can_parse_object_descriptors() {
    assert_eq!(
        Ok(Type::Object("rjvm/Test".to_string())),
        Type::parse("Lrjvm/Test;")
    );
}

#[test]
fn can_parse_array_description() {
    assert_eq!(
        Ok(Type::Array(Box::new(Type::Base(BaseType::Int)))),
        Type::parse("[I")
    );
    assert_eq!(
        Ok(Type::Array(Box::new(Type::Object(
            "java/lang/String".to_string()
        )))),
        Type::parse("[Ljava/lang/String;")
    );

    assert_eq!(
        Ok(Type::Array(Box::new(Type::Array(Box::new(
            Type::Base(BaseType::Double)
        ))))),
        Type::parse("[[D")
    );
}

#[test]
fn can_format_base_type() {
    assert_eq!("Long", format!("{}", Type::parse("J").unwrap()));
}

#[test]
fn can_format_object() {
    assert_eq!(
        "java/lang/String",
        format!("{}", Type::parse("Ljava/lang/String;").unwrap())
    );
}

#[test]
fn can_format_array() {
    assert_eq!("Int[]", format!("{}", Type::parse("[I").unwrap()));
}