use crate::command;
use command::Command;
use std::{collections::HashMap, fs::File, process::exit, io};


pub struct CommandManager {
    commands: HashMap<String, Command>,
    path: String, 
}

impl CommandManager {
    pub fn new(read_path: &String) -> Self {
        let read_result = match read_commands(read_path) {
            Ok(map) => map,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        println!("[error] 配置文件不存在!");
                    },
                    io::ErrorKind::InvalidData => {
                        println!("[error] 配置文件解析失败!");
                    },
                    _ => {
                        panic!("{:?}", e)
                    }
                };
                // 退出程序
                exit(0);
            }
        };

        CommandManager { 
            commands: read_result, 
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
fn read_commands(path: &String) -> Result<HashMap<String, Command>, io::Error> {
    // 1. 打开文件
    let file_data = File::open(path)?;
    // 2. 解析为Json对象
    let json_object: serde_json::Value = serde_json::from_reader(file_data)?;
    
    // 3. 解析json对象
    let mut cmds_map = HashMap::new();
    let cmd_values = json_object.as_array().unwrap();
    for cmd_value in cmd_values {
        let cmd: Command = match serde_json::from_value(cmd_value.clone()) {
            Ok(cmd) => cmd,
            Err(_) => continue
        };
        cmds_map.insert(cmd.name().clone(), cmd);
    }

    Ok(cmds_map)
}

