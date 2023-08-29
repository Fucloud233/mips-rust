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
            CommandPart::OP => &CommandPartInfo { length: 6, offset: 27},
            CommandPart::RS => &CommandPartInfo { length: 5, offset: 22},
            CommandPart::RT => &CommandPartInfo { length: 5, offset: 17},
            CommandPart::RD => &CommandPartInfo { length: 6, offset: 11},
            CommandPart::SHAMT => &CommandPartInfo { length: 5, offset: 6},
            CommandPart::FUNCT => &CommandPartInfo { length: 6, offset: 0},
            CommandPart::IMM => todo!(),
        }
        // COMMANDPARTS.get(self).unwrap()
    }  
    
    // 将数字转换到对应位置上
    pub fn convert_num(&self, num: u8) -> u32 {
        let res: u32 = num.into();
        res << self.info().length
    }
}

// 指令部分中的信息
pub struct CommandPartInfo {
    pub length: u8,
    pub offset: u8,
}