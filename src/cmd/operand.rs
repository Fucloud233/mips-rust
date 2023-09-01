use serde::Deserialize;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize, Clone)]
pub enum Operand {
	// INVALID = CommandType
	OP, RS, RT, RD, 
	SHAMT,
	FUNCT,
	IMM,
}

impl Operand {
    // 获取info
    pub fn info(&self) -> &OperandInfo {
        match self {
            Operand::OP => &OperandInfo { length: 6, offset: 26},
            Operand::RS => &OperandInfo { length: 5, offset: 21},
            Operand::RT => &OperandInfo { length: 5, offset: 16},
            Operand::RD => &OperandInfo { length: 5, offset: 11},
            Operand::SHAMT => &OperandInfo { length: 5, offset: 6},
            Operand::FUNCT => &OperandInfo { length: 6, offset: 0},
            Operand::IMM => &OperandInfo {length: 16, offset: 0}
        }
        // COMMANDPARTS.get(self).unwrap()
    }  
    
    // 将数字转换到对应位置上
    pub fn convert_num(&self, num: usize) -> u32 {
        // 当数组超出给定范围时 值取前32位
        let res: u32 = (num  % (1<<32)).try_into().unwrap();
        res << self.info().offset
    }

    // 验证输入数据是否超出Part大小
    pub fn check(&self, num: usize) -> bool {
        return num >= (1<<self.info().length)
    }
}

// 指令部分中的信息
pub struct OperandInfo {
    pub length: u8,
    pub offset: u8,
}