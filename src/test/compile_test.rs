use std::{fs, path::PathBuf};

use crate::compile::{parse_cmd, compile};
use crate::read::read_cmds;

#[test]
fn parse_test() {
    let line = String::from("add 1 2 3");
    let cmd = parse_cmd(&line).unwrap();
    
    let truth_oprands = vec![1, 2, 3];

    // 验证是否能够正常读取
    assert_eq!(cmd.name(), &String::from("add"));
    assert_eq!(cmd.nums().len(), truth_oprands.len());
    for i in 0..truth_oprands.len() {
        assert_eq!(cmd.nums()[i], truth_oprands[i]);
    }
}

// 解析16进制字符串
fn parse_hex_format(hex_format_text: &str) -> u32 {
    u32::from_str_radix(hex_format_text.trim(), 16).unwrap()
}

// 读取编译后的机器指令
fn read_codes(code_file_path: &PathBuf) -> Vec<u32> {
    let contents = fs::read_to_string(code_file_path).unwrap();
    let input_lines = contents.lines();

    let mut codes: Vec<u32> = Vec::new();
    for line in input_lines {
        let code: u32 = parse_hex_format(line);
        codes.push(code)
    }

    codes
}

#[test]
fn hex_parse_test() {
    let code: u32 = u32::from_str_radix("2008000B", 16).unwrap();
    assert_eq!(code, 537395211);
}

// 批量测试解析
#[test]
fn much_compile_test() {
    let cmd_file_path = PathBuf::from("data/test/test1.mips");
    let code_file_path = PathBuf::from("data/test/test1.code");
    
    // 编译代码
    let lines = read_cmds(&cmd_file_path).expect("读取失败");
    let codes = compile(&lines).expect("编译失败");

    let truth_codes = read_codes(&code_file_path);

    // 验证是否成功
    assert_eq!(codes.len(), truth_codes.len());
    for i in 0..codes.len() {
        assert_eq!(codes[i], truth_codes[i]);
    }
}