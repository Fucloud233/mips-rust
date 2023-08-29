use crate::cmd::command::Command;
use crate::cmd::command_part::CommandPart;
use crate::cmd::command_manager::CommandManager;


fn get_manager() -> CommandManager{
    let data_path = "./test_cmd.json".to_string();
    CommandManager::new(&data_path).unwrap()
}

// 新建指令测试
#[test]
fn new_cmd() {
    let cmd = Command::R {
        funct: 12,
        name: "MUT".to_string(),
        parts: vec![CommandPart::OP]
    };

    match cmd {
        Command::R {funct, ..} => assert_eq!(funct, 12),
        _ => return
    }    
}

// 读取指令测试
#[test]
fn read_cmd_test() {
    // 1. 测试读取指令
    let manager = get_manager();
    assert_eq!(manager.cmd_num(), 2);

    // 2. 测试读取指令内容1
    let cmd_name1 = "syscall".to_string();
    let cmd = manager.get(&cmd_name1).unwrap();
    assert_eq!(cmd.name(), &cmd_name1);
    
    // // 3. 测试读取指令内容2
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

// 代码转换测试
#[test]
fn to_code_test() {
    let manager = get_manager();
    
    // 生成编码
    let nums = vec![1, 2, 3];
    let add_cmd = manager.get(&"add".to_string()).unwrap();
    let code = add_cmd.to_code(&nums); 

    // add rd rs rt
    let answer = 32 + (1<<11) + (2<<22) + (3 << 17);

    // println!("code: {:#b}", code);
    assert_eq!(code, answer+1);
}

#[test]
fn convert_num_test() {
    let test_num: usize = 1<<32;
    let func = CommandPart::FUNCT.convert_num(test_num);
    println!("{}", func);
}