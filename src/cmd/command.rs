use serde::Deserialize;
use super::{
    operand::Operand,
    error::CompileErrorKind as CompileErrKind
};

// 指令类型
#[derive(Debug, PartialEq, Deserialize)]
// https://serde.rs/enum-representations.html
#[serde(tag="type")]
pub enum CmdKind {
	R {
        name: String, 
        funct: isize,
        operands: Vec<Operand>
    }, I {
        name: String,
        op: isize,
        operands: Vec<Operand>
    }, J
}

#[inline(always)]
fn embed_oprands(nums: &Vec<isize>, parts: &Vec<Operand>) -> Result<u32, CompileErrKind> {
    fn get_compile_err(nums: &Vec<isize>, parts: &Vec<Operand>) -> CompileErrKind {
        CompileErrKind::OperandNumError { 
            expected: parts.len(), found: nums.len() 
        }
    }

    let mut res:u32 = 0;
    for i in 0..parts.len() {
        // 此处不单独进行条件验证 而是采用出错报错的策略
        let part = parts.get(i).ok_or(get_compile_err(nums, parts))?;
        let num = nums.get(i).ok_or(get_compile_err(nums, parts))?;

        res += part.to_code(num);
    }

    Ok(res)
}

impl CmdKind {  
    pub fn to_code(&self, nums: &Vec<isize>) -> Result<u32, CompileErrKind> {
        match self {
            CmdKind::R{funct, operands, ..} => {
                let num = embed_oprands(nums, operands)?;
                Ok(Operand::FUNCT.to_code(funct) + num)
            },
            CmdKind::I{op, operands, ..} => {
                let num = embed_oprands(nums, operands)?;
                Ok(Operand::OP.to_code(op) + num)
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
    nums: Vec<isize>
}

impl Cmd {
    pub fn new(name: String, nums: Vec<isize>) -> Self {
        return Cmd {
            name, nums
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn nums(&self) -> &Vec<isize> {
        &self.nums
    }
}