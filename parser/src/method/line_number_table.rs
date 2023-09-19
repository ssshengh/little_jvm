use std::cmp::Ordering;

use itertools::Itertools;

use crate::utils::line_number::LineNumber;
use crate::utils::pc::ProgramCounter;

/// 模拟程序计数器和行号之间关系的表格。
/// Entries 按程序计数器排序。一个表格有两个 entry，第一个从0开始，第二个从3开始，
/// 这意味着字节码中的前三条指令对应于第1行，其余对应于第2行。
#[derive(Debug, PartialEq)]
pub struct LineNumberTable {
    entries: Vec<LineNumberTableEntry>,
}

impl LineNumberTable {
    pub fn new(entries: Vec<LineNumberTableEntry>) -> Self {
        Self {
            entries: entries.into_iter().sorted().collect(),
        }
    }

    pub fn lookup_pc(&self, pc: ProgramCounter) -> LineNumber {
        let best_matching_entry_index = match self
            .entries
            .binary_search_by(|e| e.program_counter.cmp(&pc))
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        self.entries[best_matching_entry_index].line_number
    }
}

/// Entries of a [LineNumberTable]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineNumberTableEntry {
    pub program_counter: ProgramCounter,
    pub line_number: LineNumber,
}

impl PartialOrd for LineNumberTableEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.program_counter.partial_cmp(&other.program_counter)
    }
}

impl Ord for LineNumberTableEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.program_counter.cmp(&other.program_counter)
    }
}

impl LineNumberTableEntry {
    pub fn new(program_counter: ProgramCounter, line_number: LineNumber) -> Self {
        Self {
            program_counter,
            line_number,
        }
    }
}