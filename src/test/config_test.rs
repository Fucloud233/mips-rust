use std::{path::PathBuf, env};

use crate::configure::{Config, LanguageKind, SaveFormatType};

#[test]
fn read_config_test() {
    // [注意] config读取是与运行位置的文件相关的
    // 测试环境中的可执行文件位置: target\\debug\\deps\\
    let config: Config = Config::load().unwrap();

    // 生成正确值
    let language = LanguageKind::CN;
    let mut cmds_path = env::current_exe().unwrap();
    cmds_path.pop();
    cmds_path.push("cmds_path.json");
    let default_save_format = SaveFormatType::Logisim;

    // 验证参数是否读取正确
    assert_eq!(&language, config._language());
    assert_eq!(&cmds_path, config.cmds_path());
    assert_eq!(&default_save_format, config.default_save_type());
}

#[test]
fn path_parse_test() {
    let path = "data\\test";
    
    let mut pathBuf = PathBuf::new();
    pathBuf.push("data");
    pathBuf.push("test");

    let path_buf_string = pathBuf.to_string_lossy();

    assert_eq!(path, path_buf_string)

}