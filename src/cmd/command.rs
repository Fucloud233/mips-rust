use serde::Deserialize;
use super::command_part::CommandPart;

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
    // #[serde(default = "init_nums_vec")]
    // nums: Vec<u8>
}

fn init_nums_vec() -> Vec<u8> {
    vec![]
}

impl Command {
    pub fn new(cmd_type: CommandType, 
        name: String, 
        id: u8,
        parts: Vec<CommandPart>) -> Self {

        return Command { cmd_type, id, name, parts }
    }

    pub fn to_code(&self, nums: &Vec<u8>) -> u32 {
        // 1. 验证数字是否合法

        // 2. 根据parts转换为数字

        // 3. 返回
        0
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

    // 用于验证输入数字是否合法
    fn check_nums(&self, nums: &Vec<u8>) -> bool {
        if self.parts.len() != nums.len() {
            return false
        }

        let mut i = 0;
        while i<nums.len() {
            let mut part_info = self.parts.get(i).unwrap();
            let mut scope = 1;
            // scope <<= ;
            // 判断数据是否越界
            // if nums.get(i) > scope {
            //     return false
            // } 
            i += 1;
        }

        true
    }
}
