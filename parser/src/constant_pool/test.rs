use log::info;
use crate::constant_pool::constant_pool::{ConstantPool, ConstantPoolEntry};
use crate::error::ClassFileParserError::ConstantPoolIndexToPhantomEntryError;
use crate::log::{init_log, LogLevel};

#[test]
fn test_constant_pool() {
    init_log(LogLevel::INFO);

    let mut cp = ConstantPool::default();
    cp.add_entry(ConstantPoolEntry::Utf8("hey".to_string()));
    cp.add_entry(ConstantPoolEntry::Integer(1));
    cp.add_entry(ConstantPoolEntry::Float(2.1));
    cp.add_entry(ConstantPoolEntry::Long(123));
    cp.add_entry(ConstantPoolEntry::Double(3.56));
    cp.add_entry(ConstantPoolEntry::ClassReference(1));
    cp.add_entry(ConstantPoolEntry::StringReference(1));
    cp.add_entry(ConstantPoolEntry::Utf8("joe".to_string()));
    cp.add_entry(ConstantPoolEntry::FieldReference(1, 10));
    cp.add_entry(ConstantPoolEntry::MethodReference(1, 10));
    cp.add_entry(ConstantPoolEntry::InterfaceMethodReference(1, 10));
    cp.add_entry(ConstantPoolEntry::NameAndTypeDescriptor(1, 10));

    info!("{:?}", cp);
    assert_eq!(
        cp.get_entry(1),
        Ok(&ConstantPoolEntry::Utf8("hey".to_string()))
    );
    assert_eq!(
        cp.get_entry(2),
        Ok(&ConstantPoolEntry::Integer(1))
    );
    assert_eq!(
        cp.get_entry(3),
        Ok(&ConstantPoolEntry::Float(2.1))
    );
    assert_eq!(
        cp.get_entry(4),
        Ok(&ConstantPoolEntry::Long(123))
    );
    assert_eq!(
        cp.get_entry(5),
        Err(ConstantPoolIndexToPhantomEntryError(5))
    );
    assert_eq!(
        cp.get_entry(6),
        Ok(&ConstantPoolEntry::Double(3.56))
    );
    assert_eq!(
        cp.get_entry(7),
        Err(ConstantPoolIndexToPhantomEntryError(7))
    );
    assert_eq!(ConstantPoolEntry::ClassReference(1), *cp.get_entry(8).unwrap());
    assert_eq!(ConstantPoolEntry::StringReference(1), *cp.get_entry(9).unwrap());
    assert_eq!(
        ConstantPoolEntry::Utf8("joe".to_string()),
        *cp.get_entry(10).unwrap()
    );
    assert_eq!(
        ConstantPoolEntry::FieldReference(1, 10),
        *cp.get_entry(11).unwrap()
    );
    assert_eq!(
        ConstantPoolEntry::MethodReference(1, 10),
        *cp.get_entry(12).unwrap()
    );
    assert_eq!(
        ConstantPoolEntry::InterfaceMethodReference(1, 10),
        *cp.get_entry(13).unwrap()
    );
    assert_eq!(
        ConstantPoolEntry::NameAndTypeDescriptor(1, 10),
        *cp.get_entry(14).unwrap()
    );
}