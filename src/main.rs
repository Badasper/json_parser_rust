use serde_json::Value;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("./data/data.json").expect("couldn't open file");
    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let v: Value = serde_json::from_str(&line).expect("couldn't deserialize");
        println!("Loaded value: {:?}", v["data"]);
    }
}
