#![allow(dead_code)]
use std::process::Command;
use std::fs::read_to_string;
use std::thread;
use std::time;

pub fn popen(fname: &str, args: &[&str]) -> String {
    String::from_utf8(Command::new(fname)
            .args(args)
            .output().unwrap()
            .stdout).unwrap()
}


pub fn str_from(path: &str) -> String {
    read_to_string(path).unwrap()
}


pub fn args() -> Vec<String> {
   std::env::args().collect::<Vec<String>>()
}


pub fn thread(func: fn() -> ()) -> thread::JoinHandle<()> {
    thread::spawn(move || func())
}


pub fn sleep(slp: u64) {
    let t = time::Duration::from_millis(slp);

    thread::sleep(t);
}


pub fn json_from(data: &str) -> serde_json::Value {
    // DEPENDENCIES: 
    // serde = { version = "1.0.104", features = ["derive"] }
    // serde_json = "1.0.48"
    serde_json::from_str(data).expect("Failed reading JSON file/object")
}
