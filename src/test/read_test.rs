use std::path::PathBuf;

use crate::read::read_cmds;

#[test]
fn read_test() {
    let test3_path = PathBuf::from("data/test/test3.mips");
    let lines = read_cmds(&test3_path).unwrap();

    let truth_lines = vec![
        (1, String::from("addi 1 0 1")),
        (3, String::from("addi 2 0 2")),
        (4, String::from("add 3 1 2")),
        (6, String::from("sw 3 0 0")),
        (7, String::from("syscall")),
    ];

    assert_eq!(lines.len(), truth_lines.len());
    for i in 0..lines.len() {
        assert_eq!(lines[i], truth_lines[i]);
    }
}