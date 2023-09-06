pub mod cmd {
    pub mod command;
    pub mod manager;
    pub mod operand;
    pub mod error;
}

mod compile;
mod read;
mod save;

#[cfg(test)]
mod test {
    mod cmd_test;
    mod compile_test;
}

use std::io::ErrorKind;
use compile::compile;
use read::read_cmds;
use save::{SaveFormatType, save_codes};

fn run(cmd_file_path: &String, save_file_path: &String, save_format_type: SaveFormatType) {
    // 读取文件
    let lines = match read_cmds(cmd_file_path) {
        Ok(l) => l,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => panic!("\"{}\"文件不存在", cmd_file_path),
                _ => panic!("{:?}", e)
            }
        } 
    };

    // 编译
    let codes = match compile(&lines) {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };

    // 输出
    save_codes(&codes, &save_file_path, &save_format_type);
}

fn main() {
}
