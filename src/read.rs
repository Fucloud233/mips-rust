use std::{io, fs, path::PathBuf};

// 从文件中读取输入指令
// 返回类型: 行号 + 行字符串的元组
pub fn read_cmds(file_path: &PathBuf) -> Result<Vec<(usize, String)>, io::Error> {
    let contents = fs::read_to_string(file_path)?;
    let input_lines = contents.lines();
    let mut output_lines: Vec<(usize, String)> = Vec::new(); 

    // 用于记录行号
    let mut line_num: usize = 0;
    for line in input_lines {
        // [注意] 行号如果放在后面 技术会缺
        line_num += 1;

        let tmp_line = line.trim().to_string();
        
        if !check_line(&tmp_line) {
            continue;
        }

        output_lines.push((line_num, tmp_line));
    }

    Ok(output_lines)
}

// 验证行是否有效
fn check_line(line: &String) -> bool {
    if line.is_empty() || line.starts_with("//") {
        false
    } else {
        true
    }
}