use std::process::exit;
use lazy_static::lazy_static;
use crate::{
    configure::CONFIG,
    cmd::{
        manager::CmdManager,
        command::Cmd,
        error::{
            CompileErrorKind as CompileErrKind,
            CompileError as CompileErr
        }
    }
};

fn load_manager() -> CmdManager {
    let cmds_path = CONFIG.cmds_path();

    match CmdManager::new(cmds_path) {
        Ok(m) => return m,
        Err(_) => {
            println!("\"{:?}\"不存在指令信息文件", cmds_path);
            exit(1)
        }
    }
}

lazy_static!{
    static ref MANAGER: CmdManager = load_manager();
}

// 将每一行数据解析为中间结构Cmd
pub(crate) fn parse_cmd(line: &String) -> Result<Cmd, CompileErrKind> {
    // 将指令按空白字符分解
    let mut stream = line.split_whitespace();
    
    // 1. 读取指令名称
    let cmd_name = stream.next().unwrap().to_string();
    // 2. 读取操作数
    let mut nums: Vec<isize> = Vec::new();
    loop {
        match stream.next() {
            Some(p) => {
                let num: isize = p.parse()
                    .or(Err(CompileErrKind::OperandParseError))?;
                nums.push(num)
            }
            None => break
        };
    }

    // 3. 验证是否合法
    let read_cmd = Cmd::new(cmd_name, nums);
    match MANAGER.check(&read_cmd) {
        Some(err) => return Err(err),
        None => (),
    }

    Ok(read_cmd)
}

// 修改运行函数接口: 读取指令部分被解耦开
pub fn compile(lines: &Vec<(usize, String)>) -> Result<Vec<u32>, CompileErr> {
    // 将字符串读取为Cmd中间数据结构
    let mut cmds: Vec<Cmd> = Vec::new();
    for line in lines {
        let cmd = parse_cmd(&line.1)
            .or_else(|err| Err(CompileErr::new(line.0, err)))?;
        cmds.push(cmd);
    }

    // 将字符串指令汇编成机器指令
    let mut codes: Vec<u32> = Vec::new();
    for cmd in cmds {
        let cmd_kind = MANAGER.get(cmd.name()).unwrap();
        let code = cmd_kind.to_code(cmd.nums()).unwrap();
        codes.push(code);
    }

    Ok(codes)
}
