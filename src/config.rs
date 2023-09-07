
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, env, io};

lazy_static! {
    pub static ref CONFIG: Config = Config::load().unwrap();
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum SaveFormatType {
    Logisim, Plain
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum LanguageKind {
    cn, en
}

#[derive(Deserialize, Debug)]
pub struct Config {
    language: LanguageKind,
    cmds_path: String,
    default_save_format: SaveFormatType
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        // 获取配置文件路径
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.push("config.json");

        // 解析数据并反序列化
        let file_data = File::open(path)?;
        let json_object: serde_json::Value = serde_json::from_reader(file_data)?;
        let config: Config = serde_json::from_value(json_object)?;

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
