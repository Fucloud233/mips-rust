use std::fmt::{self, Display};

use super::operand::Operand;
use thiserror::Error;

// doc: https://docs.rs/thiserror/latest/thiserror/#derives


#[derive(Error, Debug)]
pub enum CompileErrorKind {
    #[error("指令{name:?}不存在")]
    NameNotExist{name: String},
    #[error("操作数解析失败")]
    OperandParseError,
    #[error("操作数应该有{expected:?}个, 但提供了{found:?}个")]
    OperandNumError{expected: usize, found:usize},
    #[error("操作数{operand:?}超出了给定大小({expected:?}位)")]
    OperandNumExcced{operand: Operand, expected: usize}
}

#[derive(Debug)]
pub struct CompileError {
    kind: CompileErrorKind,
    line: usize
}

impl CompileError {
    pub fn new(line: usize, kind: CompileErrorKind) -> Self {
        return Self {
            line, kind
        }
    }

    pub fn kind(&self) -> &CompileErrorKind {
        &self.kind
    }

    pub fn line(&self) -> &usize {
        &self.line
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.kind)
    }
}
