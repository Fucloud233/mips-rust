use crate::config::{Config, LanguageKind, SaveFormatType};

#[test]
fn read_config_test() {
    // [注意] 测试环境中的可执行文件位置: target\\debug\\deps\\
    let config: Config = Config::load().unwrap();

    // 生成正确值
    let language = LanguageKind::cn;
    let cmds_path = String::from("./cmds_path.json");
    let default_save_format = SaveFormatType::Logisim;

    // 验证参数是否读取正确
    assert_eq!(&language, config.language());
    assert_eq!(&cmds_path, config.cmds_path());
    assert_eq!(&default_save_format, config.default_save_type());
}