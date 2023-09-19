use crate::error::ClassFileParserError;
use crate::method::descriptor::MethodDescriptor;
use crate::method::exception_table::{ExceptionTable, ExceptionTableEntry};
use crate::method::line_number_table::{LineNumberTable, LineNumberTableEntry};
use crate::utils::line_number::LineNumber;
use crate::utils::pc::ProgramCounter;
use crate::utils::types::{BaseType, Type};

#[test]
fn cannot_parse_empty_descriptor() {
    assert_cannot_parse("")
}

#[test]
fn cannot_parse_invalid_descriptor_no_arguments() {
    assert_cannot_parse("J")
}

#[test]
fn cannot_parse_invalid_descriptor_no_return_type() {
    assert_cannot_parse("(J)")
}

#[test]
fn cannot_parse_invalid_descriptor_trash_after() {
    assert_cannot_parse("()JJ")
}

fn assert_cannot_parse(descriptor: &str) {
    assert!(matches!(
            MethodDescriptor::parse(descriptor),
            Err(ClassFileParserError::InvalidMethodTypeDescriptor(s)) if s == descriptor
        ));
}

#[test]
fn can_parse_primitives() {
    assert_eq!(
        Ok(MethodDescriptor {
            parameters: vec![
                Type::Base(BaseType::Long),
                Type::Base(BaseType::Int)
            ],
            return_type: Some(Type::Base(BaseType::Double)),
        }),
        MethodDescriptor::parse("(JI)D"),
    );
}

#[test]
fn can_parse_no_args_void_return() {
    assert_eq!(
        Ok(MethodDescriptor {
            parameters: vec![],
            return_type: None,
        }),
        MethodDescriptor::parse("()V"),
    );
}

#[test]
fn can_parse_arrays_objects() {
    assert_eq!(
        Ok(MethodDescriptor {
            parameters: vec![
                Type::Object("java/lang/String".to_string()),
                Type::Base(BaseType::Int),
            ],
            return_type: Some(Type::Array(Box::new(Type::Base(BaseType::Long)))),
        }),
        MethodDescriptor::parse("(Ljava/lang/String;I)[J"),
    );
}

#[test]
fn can_format_void_to_void() {
    assert_eq!(
        "() -> void",
        format!("{}", MethodDescriptor::parse("()V").unwrap())
    );
}

#[test]
fn can_format_parameters_to_return_type() {
    assert_eq!(
        "(java/lang/String, Int) -> Long[]",
        format!(
            "{}",
            MethodDescriptor::parse("(Ljava/lang/String;I)[J").unwrap()
        )
    );
}

#[test]
fn can_get_num_arguments() {
    assert_eq!(
        2,
        MethodDescriptor::parse("(Ljava/lang/String;I)[J")
            .unwrap()
            .num_arguments(),
    );
}

#[test]
fn can_lookup_catch_handler() {
    let entry_1 = ExceptionTableEntry {
        range: ProgramCounter(0)..ProgramCounter(4),
        handler_pc: ProgramCounter(99),
        catch_class: None,
    };
    let entry_2 = ExceptionTableEntry {
        range: ProgramCounter(8)..ProgramCounter(14),
        handler_pc: ProgramCounter(88),
        catch_class: Some("java/lang/RuntimeException".to_string()),
    };
    let entry_3 = ExceptionTableEntry {
        range: ProgramCounter(13)..ProgramCounter(14),
        handler_pc: ProgramCounter(77),
        catch_class: Some("java/lang/ClassCastException".to_string()),
    };
    let table = ExceptionTable::new(vec![entry_1.clone(), entry_2.clone(), entry_3.clone()]);

    assert_eq!(vec![&entry_1], table.lookup(ProgramCounter(0)));
    assert_eq!(vec![&entry_1], table.lookup(ProgramCounter(1)));
    assert!(table.lookup(ProgramCounter(4)).is_empty());
    assert_eq!(vec![&entry_2], table.lookup(ProgramCounter(8)));
    assert_eq!(vec![&entry_2, &entry_3], table.lookup(ProgramCounter(13)));
    assert!(table.lookup(ProgramCounter(14)).is_empty());
}

#[test]
fn can_lookup_line_number() {
    let table = LineNumberTable::new(vec![
        LineNumberTableEntry::new(ProgramCounter(0), LineNumber(4)),
        LineNumberTableEntry::new(ProgramCounter(12), LineNumber(5)),
        LineNumberTableEntry::new(ProgramCounter(20), LineNumber(6)),
    ]);

    assert_eq!(LineNumber(4), table.lookup_pc(ProgramCounter(0)));
    assert_eq!(LineNumber(4), table.lookup_pc(ProgramCounter(11)));
    assert_eq!(LineNumber(5), table.lookup_pc(ProgramCounter(12)));
    assert_eq!(LineNumber(6), table.lookup_pc(ProgramCounter(20)));
    assert_eq!(LineNumber(6), table.lookup_pc(ProgramCounter(21)));
}