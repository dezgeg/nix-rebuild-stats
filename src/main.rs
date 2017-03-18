extern crate rustc_serialize;
extern crate quick_xml;

use std::collections::HashMap;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use quick_xml::reader::Reader;
use quick_xml::events::Event;

// Returns a map of attr name -> drv path, e.g: {
//  "2048-in-terminal": "/nix/store/y8ybwgsii0ws528d6gwr1m13c8jgjhsr-2048-in-terminal-2015-01-15.drv",
//   ...
//   "linuxPackages.ati_drivers_x11": "/nix/store/9kna84ddkcbm8a5945g3l23licy18ily-ati-drivers-15.12-4.9.16.drv",
//   ...
// }

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

// Returns a dependency tree in an adjacency list format of the drv_paths passed in.
// E.g. if "bastet.drv" depends on "ncurses.drv" and "boost.drv", then
// result["ncurses.drv"].contains("bastet.drv") && result["boost.drv"].contains("bastet.drv")

fn build_dep_tree(drv_paths: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut file = File::open("/tmp/j2").unwrap();
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap();

    let mut reader = Reader::from_str(file_data.as_str());
    let mut buf = Vec::new();
    reader.trim_text(true);

    let mut res = HashMap::new();
    loop {
        let event = reader.read_event(&mut buf);
        match event {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"node" => {
                        for a in e.attributes() {
                            match a {
                                Ok(a) => if a.key == b"name" {
                                    let drv = String::from_utf8(a.unescaped_value().unwrap().to_vec()).unwrap();
//                                    println!("node: {}", drv);
//                                    res.insert(drv, Vec::new());
                                    res.entry(drv).or_insert(Vec::new());
                                },
                                Err(e) => panic!("{:?}", e),
                            }
                        }
                    },
                    b"edge" => {
                        let mut src = "".to_string();
                        let mut dst = "".to_string();
                        for a in e.attributes() {
                            match a {
                                Ok(a) => {
                                    if a.key == b"src" {
                                        src = String::from_utf8(a.unescaped_value().unwrap().to_vec()).unwrap();
                                    }
                                    if a.key == b"dst" {
                                        dst = String::from_utf8(a.unescaped_value().unwrap().to_vec()).unwrap();
                                    }
                                },
                                Err(e) => panic!("{:?}", e),
                            }
                        }
//                        println!("edge: {} {}", src, dst);
                        res.entry(src).or_insert(Vec::new()).push(dst);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        //buf.clear();
    }
    res
}

fn main() {
    let attr_map = build_attr_map();
    //    for (attr, drv_path) in attr_map {
    //        println!("{}", drv_path);
    //    }
    let dep_tree = build_dep_tree(attr_map.values().map(|x| x.clone()).collect());
    for (drv, rev_deps) in dep_tree {
        println!("{} {}", rev_deps.len(), drv);
    }
}
