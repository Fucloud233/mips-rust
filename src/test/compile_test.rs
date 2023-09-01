use crate::compile::parse_cmd;

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