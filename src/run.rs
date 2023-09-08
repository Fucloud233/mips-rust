use std::{io::ErrorKind, fmt::Display};
use clap::{Parser, ValueEnum, builder::PossibleValue};
use crate::{
    compile::compile,
    read::read_cmds,
    save::save_codes,
    configure::{CONFIG, SaveFormatType}
};

// https://docs.rs/clap/latest/clap/
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

impl ValueEnum for SaveFormatType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Logisim, Self::Plain]
    }

    fn to_possible_value(&self) -> Option<PossibleValue>{
        Some(match self {
            Self::Logisim => PossibleValue::new("logisim"),
            Self::Plain => PossibleValue::new("plain")
        })
    }

    // Provided method
    // fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> { 
    // }
}

impl Display for SaveFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 
        match self {
            SaveFormatType::Logisim => "logisim",
            SaveFormatType::Plain => "plain",
        })
    }
}

#[derive(Parser, Debug)]
#[command(name = "Mips Compiler")]
#[command(version = "1.0")]
#[command(about ="将MIPS指令编译成成机器代码")]
struct Args {
    // 使用///说明参数内容

    /// 源代码位置
    source: String,
    /// 输出位置
    #[arg(short)]
    output: String,
    /// 输出格式
    #[arg(short, default_value_t = CONFIG.default_save_type().clone())]
    format: SaveFormatType
}

// 程序运行函数
pub fn run() {
    // Config::load();


    // 1. 读取参数
    let args = Args::parse();
    // print!("{:?}", args);

    // 2. 读取文件
    let lines = match read_cmds(&args.source) {
        Ok(l) => l,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => panic!("\"{}\"文件不存在", args.source),
                _ => panic!("{:?}", e)
            }
        } 
    };

    // 3. 编译
    let codes = match compile(&lines) {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };

    // 4. 输出
    save_codes(&codes, &args.output, &args.format);
}