use serde_json::{self, json, Value};
use std::{
    env::current_dir,
    fs::{create_dir_all, OpenOptions},
    io::{BufReader, Write},
};

fn assert_file() -> Result<std::fs::File, std::io::Error> {
    let file_name = "preference.json";
    let path_root = current_dir()?.join(".typhoon");
    let _path = path_root.join(file_name);
    create_dir_all(path_root.clone())?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(_path);
    file
}

fn init() -> bool {
    let mut f = assert_file().unwrap();
    let init_data = json!({});
    match write!(f, "{}", serde_json::to_string_pretty(&init_data).unwrap()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub(crate) fn get() -> serde_json::Value {
    let file = assert_file().expect("file should open read");
    let reader = BufReader::new(file);
    let json: serde_json::Value = match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(_) => {
            println!("file should be proper JSON");
            // Init content implied
            init();
            json!({})
        }
    };
    json
}


pub(crate) fn set(content: Value) -> bool {
    let mut file = assert_file().expect("file should open write");
    match write!(file, "{}", serde_json::to_string_pretty(&content).unwrap()) {
        Ok(_) => true,
        Err(_) => false,
    }
}