use std::{io, fs};
use lazy_static::lazy_static;
use crate::cmd::{
    manager::{CommandManager},
    command::{Cmd},
    error::{
        CompileErrorKind as CompileErrKind,
        CompileError as CmdErr
    }
};

lazy_static!{
    static ref read_path: String = "./data/test/test_cmd.json".to_string();
    static ref manager: CommandManager = CommandManager::new(&read_path).unwrap();
}

// 从文件中读取输入指令
// 返回类型: 行号 + 行字符串的元组
fn read_cmds(file_path: &String) -> Result<Vec<(usize, String)>, io::Error> {
    fn check_line(line: &String) -> bool {
        if line.is_empty() || line.starts_with("//") {
            false
        } else {
            true
        }
    }

    let contents = fs::read_to_string(file_path)?;
    let input_lines = contents.lines();
    let mut output_lines: Vec<(usize, String)> = Vec::new(); 

    // 用于记录行号
    let line_num: usize = 0;
    for line in input_lines {
        let tmp_line = line.trim().to_string();
        
        if !check_line(&tmp_line) {
            continue;
        }

        output_lines.push((line_num, tmp_line));
    }


    Ok(output_lines)
}

// 将每一行数据解析为中间结构Cmd
fn parse_cmd(line: &String) -> Result<Cmd, CompileErrKind> {
    // 将指令按空白字符分解
    let mut stream = line.split_whitespace();
    
    // 1. 读取指令名称
    let cmd_name = stream.next().unwrap().to_string();

    // 2. 读取操作数
    let mut nums: Vec<usize> = Vec::new();
    loop {
        match stream.next() {
            Some(p) => {
                let num: usize = p.parse()
                    .or(Err(CompileErrKind::OperandParseError))?;
                nums.push(num)
            }
            None => break
        };
    }

    // 3. 验证是否合法
    let read_cmd = Cmd::new(cmd_name, nums);
    match manager.check(&read_cmd) {
        Some(err) => return Err(err),
        None => (),
    }

    Ok(read_cmd)
}

fn output_codes(codes: &Vec<u32>, file_path: &String) {

}

pub fn run(file_path: &String, output_file: &String) -> Option<CmdErr> {
    // 读取字符串指令
    let lines = match read_cmds(file_path) {
        Ok(lines) => lines,
        // Err(e) => return Some(e)
        // FIXME: 统一各种错误类型
        Err(_) => return None
    };

    // 将字符串读取为Cmd中间数据结构
    let mut cmds: Vec<Cmd> = Vec::new();
    for line in lines {
        let cmd = match parse_cmd(&line.1) {
            Ok(c) => c,
            Err(err) => return Some(CmdErr::new(line.0, err))
        };
        cmds.push(cmd);
    }

    // TODO: 验证Cmd中间数据结构


    // TODO: 将字符串指令汇编成机器指令


    // TODO: 输出机器指令
    // output_codes(&codes, &output_file);

    None
}
