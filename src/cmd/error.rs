use super::part::CommandPart;

pub enum ErrorKind {
    NameNotExist{name:String},
    OperandParseError,
    OperandNumError{expect: usize, but:usize},
    OperandNumExcced{part: CommandPart, expect: usize, but: usize}
}

pub struct Error {
    kind: ErrorKind,
    line: usize
}

impl Error {
    pub fn new(line: usize, kind: ErrorKind) -> Self {
        return Error {
            line, kind
        }
    }
}