use lazy_static::lazy_static;
use crate::cmd::{
    manager::{CommandManager},
    command::{Cmd},
    error::{
        CompileErrorKind as CompileErrKind,
        CompileError as CompileErr
    }
};

fn load_manager() -> CommandManager {
    let read_path = "./data/test/test_cmd.json".to_string();
    match CommandManager::new(&read_path) {
        Ok(m) => return m,
        Err(err) => panic!("{:?}", err)
    }
}

lazy_static!{
    static ref MANAGER: CommandManager = load_manager();
}

// 将每一行数据解析为中间结构Cmd
pub(crate) fn parse_cmd(line: &String) -> Result<Cmd, CompileErrKind> {
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
    match MANAGER.check(&read_cmd) {
        Some(err) => return Err(err),
        None => (),
    }

    Ok(read_cmd)
}

fn output_codes(codes: &Vec<u32>, file_path: &String) {

}

// 修改运行函数接口: 读取指令部分被解耦开
pub fn run(lines: &Vec<(usize, String)>, output_file: &String) -> Option<CompileErr> {
    // 将字符串读取为Cmd中间数据结构
    let mut cmds: Vec<Cmd> = Vec::new();
    for line in lines {
        let cmd = match parse_cmd(&line.1) {
            Ok(c) => c,
            Err(err) => return Some(CompileErr::new(line.0, err))
        };
        cmds.push(cmd);
    }

    // TODO: 验证Cmd中间数据结构


    // TODO: 将字符串指令汇编成机器指令


    // TODO: 输出机器指令
    // output_codes(&codes, &output_file);

    None
}
