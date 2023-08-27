use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::Deserialize;

// 
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
#[derive(Debug, Deserialize)]
pub enum CommandType {
	R, I, J
}


#[derive(Debug, Deserialize)]
pub struct Command {
    cmd_type: CommandType,
    id: u8,
    name: String,
    parts: Vec<CommandPart>,
    #[serde(default = "init_nums_vec")]
    nums: Vec<u8>
}

fn init_nums_vec() -> Vec<u8> {
    vec![]
}

impl Command {
    pub fn new(cmd_type: CommandType, 
        name: String, 
        id: u8,
        parts: Vec<CommandPart>, 
        nums: Vec<u8>) -> Self {

        return Command { cmd_type, id, name, parts, nums }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn parts(&self) -> &Vec<CommandPart> {
        &self.parts
    }
}