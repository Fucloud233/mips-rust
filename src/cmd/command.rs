use serde::Deserialize;
use crate::cmd::command_part::CommandPart;

// 指令类型
#[derive(Debug, PartialEq, Deserialize)]
// https://serde.rs/enum-representations.html
#[serde(tag="type")]
pub enum Command {
	R {
        funct: usize,
        name: String, 
        parts: Vec<CommandPart>
    }, I, J
}


impl Command {
    pub fn to_code(&self, nums: &Vec<usize>) -> u32 {

        match self {
            Command::R{funct, parts, ..} => {
                if parts.len() != nums.len() {
                    return 0;
                }
                
                let mut res = CommandPart::FUNCT.convert_num(*funct);
                for i in 0..parts.len() {
                    res += parts[i].convert_num(nums[i]);
                }

                res
            },
            Command::I => todo!(),
            Command::J => todo!(),
        }
    } 

    pub fn name(&self) -> &String {
        match self {
            Command::R{ name, ..} => name,
            Command::I => todo!(),
            Command::J => todo!(),
        }
    }

    pub fn parts(&self) -> &Vec<CommandPart> {
        match self {
            Command::R{ parts, ..} => parts,
            Command::I => todo!(),
            Command::J => todo!(),
        }
    }
}

// // 用于验证输入数字是否合法
// fn check_nums(&self, nums: &Vec<u8>) -> bool {
//     if self.parts.len() != nums.len() {
//         return false
//     }

//     let mut i = 0;
//     while i<nums.len() {
//         let part_len =  self.parts[i].info().length();
//         let scope = 1 << part_len;
//         // 判断数据是否越界
//         if nums[i] > scope {
//             return false
//         } 
//         i += 1;
//     }

//     true
// }