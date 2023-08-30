use serde::Deserialize;
use crate::cmd::command_part::CommandPart;
use std::io;

// 指令类型
#[derive(Debug, PartialEq, Deserialize)]
// https://serde.rs/enum-representations.html
#[serde(tag="type")]
pub enum Command {
	R {
        name: String, 
        funct: usize,
        parts: Vec<CommandPart>
    }, I {
        name: String,
        op: usize,
        parts: Vec<CommandPart>
    }, J
}

#[inline(always)]
fn embed_num(nums: &Vec<usize>, parts: &Vec<CommandPart>) -> Result<u32, io::Error> {
    if parts.len() != nums.len() {
        // 当非法输入时
        return Err(io::ErrorKind::InvalidInput.into())
    }

    let mut res:u32 = 0;
    for i in 0..parts.len() {
        res += parts[i].convert_num(nums[i]);
    }

    Ok(res)
}

impl Command {
    pub fn to_code(&self, nums: &Vec<usize>) -> Result<u32, io::Error> {
        match self {
            Command::R{funct, parts, ..} => {
                let num = embed_num(nums, parts)?;
                Ok(CommandPart::FUNCT.convert_num(*funct) + num)
            },
            Command::I{op, parts, ..} => {
                let num = embed_num(nums, parts)?;
                Ok(CommandPart::OP.convert_num(*op) + num)
            }
            Command::J => todo!(),
        }
    } 

    pub fn name(&self) -> &String {
        match self {
            Command::R{ name, ..} => name,
            Command::I{ name, ..} => name,
            Command::J => todo!(),
        }
    }

    pub fn parts(&self) -> &Vec<CommandPart> {
        match self {
            Command::R{ parts, ..} => parts,
            Command::I{ parts, ..} => parts,
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