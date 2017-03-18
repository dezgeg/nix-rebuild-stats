extern crate rustc_serialize;

use std::collections::HashMap;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

fn build_attr_map() -> HashMap<String, String> {
    let mut file = File::open("/tmp/j").unwrap();
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap();

    let json = Json::from_str(file_data.as_str()).unwrap();

    let mut res = HashMap::new();
    for pair in json.as_array().unwrap() {
        let arr = pair.as_array().unwrap();
        let attr = arr[0].as_string().unwrap().to_string();
        let drv_path = arr[1].as_string().unwrap().to_string();
        res.insert(attr, drv_path);
    }
    return res;
}

fn main() {
    println!("{:?}", build_attr_map());
}
