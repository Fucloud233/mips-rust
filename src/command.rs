use std::collections::HashMap;
use lazy_static::lazy_static;

// 
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CommandPart {
	// INVALID = CommandType
	OP, RS, RT, RD, 
	SHAMT,
	FUNCT,
	IMM,
}

// 指令部分中的信息
struct CommandPartInfo {
    length: u8,
    offset: u8,
}

lazy_static! {
    static ref CommandParts: HashMap<CommandPart, CommandPartInfo> = init_command_parts();
}

fn init_command_parts() -> HashMap<CommandPart, CommandPartInfo> {
    let mut command_parts = HashMap::new();
    command_parts.insert(CommandPart::OP, CommandPartInfo { length: 6, offset:  27});
    command_parts.insert(CommandPart::RS, CommandPartInfo { length: 5, offset:  22});
    command_parts.insert(CommandPart::RT, CommandPartInfo { length: 5, offset:  17});
    command_parts.insert(CommandPart::RD, CommandPartInfo { length: 6, offset:  11});
    command_parts.insert(CommandPart::SHAMT, CommandPartInfo { length: 5, offset:  6});
    command_parts.insert(CommandPart::FUNCT, CommandPartInfo { length: 6, offset:  0});

    command_parts
}

// 指令类型
#[derive(Debug)]
pub enum CommandType {
	R, I, J
}


#[derive(Debug)]
pub struct Command {
    cmd_type: CommandType,
    name: String,
    parts: Vec<CommandPart>,
    nums: Vec<u8>
}

impl Command {
    pub fn new(cmd_type: CommandType, 
        name: String, 
        parts: Vec<CommandPart>, 
        nums: Vec<u8>) -> Self {

        return Command { cmd_type, name, parts, nums }
    }
}