use crate::cmd::command::{Command, CommandType};
use crate::cmd::command_part::CommandPart;
use crate::cmd::command_manager::CommandManager;


// 新建指令测试
#[test]
fn new_cmd() {
    let cmd = Command::new(
        CommandType::R, 
        "MUT".to_string(),
        12,
        vec![CommandPart::OP],
    );
    // println!("{:?}", cmd)
    assert_eq!(cmd.cmd_type(), &CommandType::R)
}

// 读取指令测试
#[test]
fn read_cmd() {
    // 1. 测试读取指令
    let data_path = "./test_cmd.json".to_string();
    let manager = CommandManager::new(&data_path);
    assert_eq!(manager.cmd_num(), 2);

    // 2. 测试读取指令内容1
    let cmd_name1 = "syscall".to_string();
    let cmd = manager.get(&cmd_name1).unwrap();
    assert_eq!(cmd.name(), &cmd_name1);
    
    // 3. 测试读取指令内容2
    let cmd_name2 = "add".to_string();
    let cmd = manager.get(&cmd_name2).unwrap();
    let parts = cmd.parts();
    let true_parts = [CommandPart::RD, CommandPart::RS, CommandPart::RT];
    let mut i = 0;
    while i < parts.len() {
        assert_eq!(parts[i], true_parts[i]);
        i += 1;
    }
}