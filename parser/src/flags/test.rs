use bitflags::Flags;
use crate::flags::{ClassAccessFlags, FieldFlags, MethodFlags};

#[test]
fn test_flags() {
    let class_flag = ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE;
    assert_eq!(class_flag.bits(), 0x0201);
    assert_eq!(format!("{:?}", class_flag.0), "PUBLIC | INTERFACE");

    let filed_flag = FieldFlags::PUBLIC | FieldFlags::FINAL;
    print!("{}", filed_flag.0);
}