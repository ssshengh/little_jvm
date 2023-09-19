use std::fmt;
use std::fmt::{Debug, Display, Formatter, write};
use log::debug;
use crate::constant_pool::constant_pool::ConstantPoolSlot::{Entry, PhantomEntry};
use crate::error::{ClassFileParserError, ClassFileParserResult};

/// 常量池的类型，目前支持了 11 个，参考文档:
/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[derive(Debug, PartialEq)]
pub enum ConstantPoolEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassReference(u16),
    StringReference(u16),
    FieldReference(u16, u16),
    MethodReference(u16, u16),
    InterfaceMethodReference(u16, u16),
    NameAndTypeDescriptor(u16, u16),
}


/// 常量池, 需要注意的是, 常量池的索引从 1 开始, 而不是 0!
/// 常量池的极限大小是两个字节, u16
#[derive(Default)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolSlot>,
}

/// Constants in the pool generally take one slot, but long and double take two. We do not use
/// the second one, so we have a tombstone to ensure the indexes match.
/// 这里参考了 Andrew 的处理策略, 使用另一个值来表示空的一个常量位, 确保 long 和 double 的第二位对应
/// 但是实际上在 Rust 结构体中存储的数据是仅存储在第一位上的
#[derive(Debug)]
enum ConstantPoolSlot {
    Entry(ConstantPoolEntry),
    // 幽灵 entry，意味着这个 entry 被使用但是实际上不可访问, 也即事实上是持有了这个 entry 的
    PhantomEntry(),
}

impl ConstantPool {
    /// 添加一个 entry, 如果是 long 或者 double, 则额外添加一个
    pub fn add_entry(&mut self, entry: ConstantPoolEntry) {
        let should_insert_phantom = matches!(
            &entry,
            ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_)
        );

        self.entries.push(Entry(entry));

        if should_insert_phantom {
            self.entries.push(PhantomEntry());
        }
    }

    /// 获取一个 entry, 注意, JVM 规定常量池的索引从 1 开始, 而不是 0
    pub fn get_entry(&self, index: u16) -> ClassFileParserResult<&ConstantPoolEntry> {
        if index == 0 || index as usize > self.entries.len()  {
            return Err(ClassFileParserError::WrongConstantPoolIndexError(index));
        }

        let entry = &self.entries[index as usize - 1];
        match entry {
            PhantomEntry() => Err(ClassFileParserError::ConstantPoolIndexToPhantomEntryError(index)),
            Entry(entry) => Ok(entry),
        }
    }

    /// 组织 entry, 从中可以看到几个 reference 数组的含义, 本质上还是指向了常量池。
    fn fmt_entry(&self, idx: u16) -> ClassFileParserResult<String> {
        let entry = self.get_entry(idx)?;
        let text = match entry {
            ConstantPoolEntry::Utf8(ref s) => format!("String: \"{s}\""),
            ConstantPoolEntry::Integer(n) => format!("Integer: {n}"),
            ConstantPoolEntry::Float(n) => format!("Float: {n}"),
            ConstantPoolEntry::Long(n) => format!("Long: {n}"),
            ConstantPoolEntry::Double(n) => format!("Double: {n}"),
            ConstantPoolEntry::ClassReference(n) => {
                format!("ClassReference: {} => ({})", n, self.fmt_entry(*n)?)
            }
            ConstantPoolEntry::StringReference(n) => {
                format!("StringReference: {} => ({})", n, self.fmt_entry(*n)?)
            }
            ConstantPoolEntry::FieldReference(i, j) => {
                format!(
                    "FieldReference: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(*i)?,
                    self.fmt_entry(*j)?
                )
            }
            ConstantPoolEntry::MethodReference(i, j) => {
                format!(
                    "MethodReference: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(*i)?,
                    self.fmt_entry(*j)?
                )
            }
            ConstantPoolEntry::InterfaceMethodReference(i, j) => {
                format!(
                    "InterfaceMethodReference: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(*i)?,
                    self.fmt_entry(*j)?
                )
            }
            &ConstantPoolEntry::NameAndTypeDescriptor(i, j) => {
                format!(
                    "NameAndTypeDescriptor: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(i)?,
                    self.fmt_entry(j)?
                )
            }
        };
        Ok(text)
    }

    pub fn text_of(&self, idx: u16) -> ClassFileParserResult<String> {
        let entry = self.get_entry(idx)?;
        let text = match entry {
            ConstantPoolEntry::Utf8(ref s) => s.clone(),
            ConstantPoolEntry::Integer(n) => n.to_string(),
            ConstantPoolEntry::Float(n) => n.to_string(),
            ConstantPoolEntry::Long(n) => n.to_string(),
            ConstantPoolEntry::Double(n) => n.to_string(),
            ConstantPoolEntry::ClassReference(n) => self.text_of(*n)?,
            ConstantPoolEntry::StringReference(n) => self.text_of(*n)?,
            ConstantPoolEntry::FieldReference(i, j) => {
                format!("{}.{}", self.text_of(*i)?, self.text_of(*j)?)
            }
            ConstantPoolEntry::MethodReference(i, j) => {
                format!("{}.{}", self.text_of(*i)?, self.text_of(*j)?)
            }
            ConstantPoolEntry::InterfaceMethodReference(i, j) => {
                format!("{}.{}", self.text_of(*i)?, self.text_of(*j)?)
            }
            ConstantPoolEntry::NameAndTypeDescriptor(i, j) => {
                format!("{}: {}", self.text_of(*i)?, self.text_of(*j)?)
            }
        };
        Ok(text)
    }
}


impl Debug for ConstantPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Constant pool: (size: {})", self.entries.len())?;
        for (raw_idx, _) in self.entries.iter().enumerate() {
            let index = (raw_idx + 1) as u16;
            let entry_text = self
                .fmt_entry(index);
            match entry_text {
                Ok(str) => {
                    writeln!(f, "    {}, {}", index, str)?;
                }
                Err(_) => {
                    writeln!(f, "    {}, ------ PhantomEntry ------", index)?;
                }
            }
        }
        Ok(())
    }
}