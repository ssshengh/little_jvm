use std::ops::Range;

use crate::utils::pc::ProgramCounter;

/// Exception table of a method's code
#[derive(Debug, Default, PartialEq)]
pub struct ExceptionTable {
    entries: Vec<ExceptionTableEntry>,
}

impl ExceptionTable {
    pub fn new(entries: Vec<ExceptionTableEntry>) -> Self {
        Self { entries }
    }

    pub fn lookup(&self, pc: ProgramCounter) -> Vec<&ExceptionTableEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.range.contains(&pc))
            .collect()
    }
}

/// Entries of the exception table
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExceptionTableEntry {
    /// The range of program counters that this entry covers
    pub range: Range<ProgramCounter>,
    /// The address of the handler of this entry
    pub handler_pc: ProgramCounter,
    /// The class or superclass that matches this entry
    pub catch_class: Option<String>,
}