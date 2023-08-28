use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
// #[serde(untagged)]
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

impl CommandPartInfo{
    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn offset(&self) -> u8 {
        self.offset
    }
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

pub fn get_cmd_part_info(part_type: &CommandPart) -> Option<&CommandPartInfo> {
    CommandParts.get(part_type)
} 