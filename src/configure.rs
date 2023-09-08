
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, path::PathBuf, env, io};

lazy_static! {
    pub static ref CONFIG: Config = Config::load().unwrap();
    static ref CONFIG_FILE_NAME: String =  String::from("Config.json");
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum SaveFormatType {
    Logisim, Plain
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum LanguageKind {
    CN, EN
}


// PathBuf instead of Path
// PathBuf to Path is as String to str
#[derive(Deserialize, Debug)]
pub struct Config {
    // TODO: 后续可以支持中英多语言
    #[serde(rename = "language")]
    _language: LanguageKind,
    default_save_format: SaveFormatType,
    // 指令信息存储路径
    cmds_path: PathBuf,
    // 程序的运行路径 (与工作路径不同)
    #[serde(skip)]
    run_path: PathBuf
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        // 获取配置文件路径
        let mut run_path = env::current_exe().unwrap();
        run_path.pop();

        // 获取配置文件路径
        let mut config_path = run_path.clone();
        config_path.push(CONFIG_FILE_NAME.as_str());
        // 解析配置信息 (反序列化)
        let file_data = File::open(config_path)?;
        let json_object: serde_json::Value = serde_json::from_reader(file_data)?;
        let mut config: Config = serde_json::from_value(json_object)?;

        config.run_path = run_path;
        
        // 将相对路径转换为绝对路径
        if !config.cmds_path.is_absolute() {
            config.cmds_path = to_absolute_path(&config.run_path, config.cmds_path);
        }

        Ok(config)
    }

    pub fn _language(&self) -> &LanguageKind {
        &self._language
    }

    pub fn cmds_path(&self) -> &PathBuf {
        &self.cmds_path
    }

    pub fn default_save_type(&self) -> &SaveFormatType {
        &self.default_save_format
    }
}

#[inline(always)]
fn to_absolute_path(start_path: &PathBuf, rel_path: PathBuf) -> PathBuf {
    let mut abs_path = start_path.clone();
    abs_path.push(rel_path);

    return abs_path
}