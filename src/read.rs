use std::{io, fs};

// 从文件中读取输入指令
// 返回类型: 行号 + 行字符串的元组
pub fn read_cmds(file_path: &String) -> Result<Vec<(usize, String)>, io::Error> {
    fn check_line(line: &String) -> bool {
        if line.is_empty() || line.starts_with("//") {
            false
        } else {
            true
        }
    }

    let contents = fs::read_to_string(file_path)?;
    let input_lines = contents.lines();
    let mut output_lines: Vec<(usize, String)> = Vec::new(); 

    // 用于记录行号
    let mut line_num: usize = 0;
    for line in input_lines {
        let tmp_line = line.trim().to_string();
        
        if !check_line(&tmp_line) {
            continue;
        }

        output_lines.push((line_num, tmp_line));
        line_num += 1;
    }

    Ok(output_lines)
}