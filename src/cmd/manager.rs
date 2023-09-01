use std::{collections::HashMap, fs::File, io};
use super::{
    error::CompileErrorKind as CompileErrKind,
    command::{CmdKind, Cmd}
};

pub struct CommandManager {
    // 使用枚举类型Command来存储Command信息
    // 能够更方便地映射
    commands: HashMap<String, CmdKind>,
    // path: String, 
}

impl CommandManager {
    // CommandManager在新建之后 就会读取配置文件
    // 可能会遇到以下错误
    // 1. NotFound: 配置文件不存在
    // 2. InvalidData: 配置文件解析失败
    pub fn new(cmd_file_path: &String) -> Result<Self, io::Error> {
        let read_result = read_commands(cmd_file_path)?;

        Ok(CommandManager { 
            commands: read_result, 
            // path: read_path.clone()
        })
    }

    pub fn get(&self, cmd_name: &String) -> Option<&CmdKind> {
        // 转换为小写 提高鲁棒性
        self.commands.get(&cmd_name.to_lowercase())
    }

    // 验证输入指令是否合法
    pub fn check(&self, cmd: &Cmd) -> Option<CompileErrKind> {
        // 验证指令名称是否存在 
        let command = match self.get(cmd.name()) {
            Some(c) => c, 
            // FIXME: 这里是否要clone 还是修改引号
            None => return Some(CompileErrKind::NameNotExist { name: cmd.name().clone()})
        };

        // 验证操作数是否相同
        let operands = command.operands();
        let nums = cmd.nums();
        if operands.len() != nums.len() {
            return Some(CompileErrKind::OperandNumError { 
                expected: operands.len(), 
                found: nums.len() 
            })
        }

        // 验证操作数是否在大小限制内
        for i in 0..operands.len() {
            if operands[i].check(nums[i]) {
                return Some(CompileErrKind::OperandNumExcced { 
                    // FIXME: 这里是否要clone 还是修改引号
                    operand: operands[i].clone(), 
                    expected: operands.len(), 
                    found: nums.len() 
                })
            }
        }
        
        None
    }

    pub fn cmd_num(&self) -> usize {
        self.commands.len()
    }

    pub fn is_exist(cmd: String) -> bool {
        false
    }
}


// https://www.qttc.net/509-rust-parse-json.html
fn read_commands(path: &String) -> Result<HashMap<String, CmdKind>, io::Error> {
    // 1. 打开文件
    let file_data = File::open(path)?;

    // 2. 解析为Json对象
    let json_object: serde_json::Value = serde_json::from_reader(file_data)?;
    
    // 3. 解析Json对象
    let mut cmds_map = HashMap::new();
    let cmd_values = json_object.as_array().unwrap();
    for cmd_value in cmd_values {
        // 获取指令名称
        let name = cmd_value.get("name").unwrap()
            .as_str().unwrap().to_string();
        
        // 获取指令
        let cmd: CmdKind = match serde_json::from_value(cmd_value.clone()) {
            Ok(cmd) => cmd,
            Err(_) => continue
        };
        cmds_map.insert(name, cmd);
    }

    Ok(cmds_map)
}