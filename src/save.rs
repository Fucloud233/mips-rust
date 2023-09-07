use std::{fs::File, io::Write};

// use lazy_static::lazy_static;
// lazy_static! {
//     static ref LOGISIM_HEADER: String = String::from("v2.0 raw");
// }

#[derive(Debug, Clone)]
pub enum SaveFormatType {
    Logisim, Plain
}

fn save_as_logisim(codes: &Vec<u32>, file_path: &String) {
    let logisim_header = String::from("v2.0 raw");

    let mut file = File::create(file_path).unwrap();
    let _ = file.write_fmt(format_args!("{}\n", logisim_header));

    for code in codes {
        let _ = file.write_fmt(format_args!("{:08x}\n", code));
    }
}

fn save_as_plain(codes: &Vec<u32>, file_path: &String) {
} 


pub fn save_codes(codes: &Vec<u32>, file_path: &String, save_format_type: &SaveFormatType) {
    match save_format_type {
        SaveFormatType::Logisim => save_as_logisim(codes, file_path),
        SaveFormatType::Plain => save_as_plain(codes, file_path)
    }
}
