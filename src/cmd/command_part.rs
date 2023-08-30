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

impl CommandPart {
    // 获取info
    pub fn info(&self) -> &CommandPartInfo {
        match self {
            CommandPart::OP => &CommandPartInfo { length: 6, offset: 26},
            CommandPart::RS => &CommandPartInfo { length: 5, offset: 21},
            CommandPart::RT => &CommandPartInfo { length: 5, offset: 16},
            CommandPart::RD => &CommandPartInfo { length: 5, offset: 11},
            CommandPart::SHAMT => &CommandPartInfo { length: 5, offset: 6},
            CommandPart::FUNCT => &CommandPartInfo { length: 6, offset: 0},
            CommandPart::IMM => &CommandPartInfo {length: 17, offset: 0}
        }
        // COMMANDPARTS.get(self).unwrap()
    }  
    
    // 将数字转换到对应位置上
    pub fn convert_num(&self, num: usize) -> u32 {
        // 当数组超出给定范围时 值取前32位
        let res: u32 = (num  % (1<<32)).try_into().unwrap();
        res << self.info().offset
    }
}

// 指令部分中的信息
pub struct CommandPartInfo {
    pub length: u8,
    pub offset: u8,
}