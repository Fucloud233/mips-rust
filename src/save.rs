use std::{fs::File, io::Write, path::PathBuf};
use crate::configure::SaveFormatType;

// use lazy_static::lazy_static;
// lazy_static! {
//     static ref LOGISIM_HEADER: String = String::from("v2.0 raw");
// }

fn save_as_logisim(codes: &Vec<u32>, file_path: &PathBuf) {
    let logisim_header = String::from("v2.0 raw");

    let mut file = File::create(file_path).unwrap();
    let _ = file.write_fmt(format_args!("{}\n", logisim_header));

    for code in codes {
        let _ = file.write_fmt(format_args!("{:08x}\n", code));
    }
}

fn save_as_plain(_codes: &Vec<u32>, _file_path: &PathBuf) {
} 


pub fn save_codes(codes: &Vec<u32>, file_path: &PathBuf, save_format_type: &SaveFormatType) {
    match save_format_type {
        SaveFormatType::Logisim => save_as_logisim(codes, file_path),
        SaveFormatType::Plain => save_as_plain(codes, file_path)
    }
}
