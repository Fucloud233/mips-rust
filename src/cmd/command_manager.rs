use std::{collections::HashMap, fs::File, io};
use super::command::Command;

pub struct CommandManager {
    // 使用枚举类型Command来存储Command信息
    // 能够更方便地映射
    commands: HashMap<String, Command>,
    // path: String, 
}

impl CommandManager {
    // CommandManager在新建之后 就会读取配置文件
    // 可能会遇到以下错误
    // 1. NotFound: 配置文件不存在
    // 2. InvalidData: 配置文件解析失败
    pub fn new(read_path: &String) -> Result<Self, io::Error> {
        let read_result = read_commands(read_path)?;

        Ok(CommandManager { 
            commands: read_result, 
            // path: read_path.clone()
        })
    }

    pub fn get(&self, cmd: &String) -> Option<&Command> {
        // 转换为小写 提高鲁棒性
        self.commands.get(&cmd.to_lowercase())
    }

    pub fn cmd_num(&self) -> usize {
        self.commands.len()
    }

    pub fn is_exist(cmd: String) -> bool {
        false
    }
}


// https://www.qttc.net/509-rust-parse-json.html
fn read_commands(path: &String) -> Result<HashMap<String, Command>, io::Error> {
    // 1. 打开文件
    let file_data = File::open(path)?;
    // 2. 解析为Json对象
    let json_object: serde_json::Value = serde_json::from_reader(file_data)?;
    
    // 3. 解析json对象
    let mut cmds_map = HashMap::new();
    let cmd_values = json_object.as_array().unwrap();
    for cmd_value in cmd_values {
        // 获取指令名称
        let name = cmd_value.get("name").unwrap()
            .as_str().unwrap().to_string();
        
        // 获取指令
        let cmd: Command = match serde_json::from_value(cmd_value.clone()) {
            Ok(cmd) => cmd,
            Err(_) => continue
        };
        cmds_map.insert(name, cmd);
    }

    Ok(cmds_map)
}
