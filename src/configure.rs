
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, path::PathBuf, env, io};

lazy_static! {
    pub static ref CONFIG: Config = Config::load().unwrap();
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum SaveFormatType {
    Logisim, Plain
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum LanguageKind {
    CN, EN
}

#[derive(Deserialize, Debug)]
pub struct Config {
    language: LanguageKind,
    cmds_path: String,
    default_save_format: SaveFormatType,
    #[serde(skip)]
    run_path: PathBuf
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        // 获取配置文件路径
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.push("config.json");

        // 解析数据并反序列化
        let file_data = File::open(&path)?;
        let json_object: serde_json::Value = serde_json::from_reader(file_data)?;
        let mut config: Config = serde_json::from_value(json_object)?;

        
        // 再处理
        // if PathBuf::new(config.cmds_path) {

        // };
        config.run_path = path;

        Ok(config)
    }

    pub fn language(&self) -> &LanguageKind {
        &self.language
    }

    pub fn cmds_path(&self) -> &String {
        &self.cmds_path
    }

    pub fn default_save_type(&self) -> &SaveFormatType {
        &self.default_save_format
    }
}
