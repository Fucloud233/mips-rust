use crate::cmd::command::Command;
use crate::cmd::command_part::CommandPart;
use crate::cmd::command_manager::CommandManager;


fn get_manager(data_path: String) -> CommandManager{
    // let data_path = "./test_cmd.json".to_string();
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
    let manager = get_manager("./test_cmd.json".to_string());
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
fn r_to_code_test() {
    let manager = get_manager("./test_cmd.json".to_string());
    
    /* R型指令测试 */
    // 生成编码
    let nums = vec![1, 2, 3];
    let add_cmd = manager.get(&"add".to_string()).unwrap();
    let code = add_cmd.to_code(&nums).unwrap(); 

    // add rd rs rt
    let answer = 32 + (1<<11) + (2<<21) + (3 << 16);

    // println!("code: {:#b}", code);
    assert_eq!(code, answer);
}

#[test]
fn i_to_code_test() {
    let manager = get_manager("./test_i_cmd.json".to_string());
    assert_eq!(manager.cmd_num(), 1);

    
    let nums = vec![1, 2, 3];
    let beq_cmd = manager.get(&"beq".to_string()).unwrap();
    let code = beq_cmd.to_code(&nums).unwrap();

    let answer = (4 << 26) + (1 <<21) + (2 << 16) + 3;
    assert_eq!(code, answer);
}

#[test]
fn convert_num_test() {
    let test_num: usize = 1<<32;
    let func = CommandPart::FUNCT.convert_num(test_num);
    println!("{}", func);
}