use super::operand::Operand;
use thiserror::Error;

// doc: https://docs.rs/thiserror/latest/thiserror/#derives

#[derive(Error, Debug)]
pub enum ManagerErrorKind {
    #[error("{name:?}文件不存在")]
    NotFound{name: String},
    #[error("Json文件解析错误")]
    JsonParseError,    
}

#[derive(Error, Debug)]
pub enum CompileErrorKind {
    #[error("指令\"{name:?}\"不存在")]
    NameNotExist{name:String},
    #[error("操作数解析失败")]
    OperandParseError,
    #[error("操作数应该有{expected:?}个, 但提供了{found:?}")]
    OperandNumError{expected: usize, found:usize},
    #[error("操作数{operand:?}大小为{expected:?}位")]
    OperandNumExcced{operand: Operand, expected: usize, found: usize}
}

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
}
