use crate::command;
use command::Command;
use std::{collections::HashMap, fs::File, alloc::System, process::exit};


pub struct CommandManager {
    commands: HashMap<String, Command>,
    path: String, 
}

impl CommandManager {
    pub fn new(read_path: &String) -> Self {
        let commands = read_commands(read_path);

        CommandManager { 
            commands: commands, 
            path: read_path.clone()
        }
    }

    pub fn get_command(self: &Self, cmd: &String) -> Option<&Command> {
        return self.commands.get(cmd);
    }

    fn is_exist(cmd: String) -> bool {
        false
    }
}


// https://www.qttc.net/509-rust-parse-json.html
fn read_commands(path: &String) -> HashMap<String, Command> {
    // 1. 打开文件
    let file_data = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("配置文件不存在!");
            exit(-1);
        }
    };

    // 2. 解析为Json对象
    let json_object: serde_json::Value = match serde_json::from_reader(file_data) {
        Ok(object) => object,
        Err(_) => {
            println!("配置文件解析失败!");
            return HashMap::new();
        }
    };
    
    // 3. 解析json对象
    let mut cmds_map = HashMap::new();
    let cmd_values = json_object.as_array().unwrap();
    for cmd_value in cmd_values {
        let cmd: Command = match serde_json::from_value(cmd_value.clone()) {
            Ok(cmd) => cmd,
            Err(e) => {
                return HashMap::new();
            }
        };
        cmds_map.insert(cmd.name().clone(), cmd);
    }

    cmds_map
}

