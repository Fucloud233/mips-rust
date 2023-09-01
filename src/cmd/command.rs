use serde::Deserialize;
use crate::cmd::operand::Operand;
use std::io;

// 指令类型
#[derive(Debug, PartialEq, Deserialize)]
// https://serde.rs/enum-representations.html
#[serde(tag="type")]
pub enum CmdKind {
	R {
        name: String, 
        funct: usize,
        operands: Vec<Operand>
    }, I {
        name: String,
        op: usize,
        operands: Vec<Operand>
    }, J
}

#[inline(always)]
fn embed_num(nums: &Vec<usize>, parts: &Vec<Operand>) -> Result<u32, io::Error> {
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

impl CmdKind {
    pub fn to_code(&self, nums: &Vec<usize>) -> Result<u32, io::Error> {
        match self {
            CmdKind::R{funct, operands, ..} => {
                let num = embed_num(nums, operands)?;
                Ok(Operand::FUNCT.convert_num(*funct) + num)
            },
            CmdKind::I{op, operands, ..} => {
                let num = embed_num(nums, operands)?;
                Ok(Operand::OP.convert_num(*op) + num)
            }
            CmdKind::J => todo!(),
        }
    } 

    pub fn name(&self) -> &String {
        match self {
            CmdKind::R{ name, ..} => name,
            CmdKind::I{ name, ..} => name,
            CmdKind::J => todo!(),
        }
    }

    pub fn operands(&self) -> &Vec<Operand> {
        match self {
            CmdKind::R{ operands, ..} => operands,
            CmdKind::I{ operands, ..} => operands,
            CmdKind::J => todo!(),
        }
    }
}

// 输入的指令结构体
pub struct Cmd {
    name: String,
    nums: Vec<usize>
}

impl Cmd {
    pub fn new(name: String, nums: Vec<usize>) -> Self {
        return Cmd {
            name, nums
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn nums(&self) -> &Vec<usize> {
        &self.nums
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