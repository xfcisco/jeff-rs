#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use modules :: popen;
use std :: thread;
use std :: path :: Path;

fn main() {
    let mut result: Status;

    loop {
        result = BootSequence :: Start();
        System :: Check(&result, "boot sequence");

        result = System :: Init();
        System :: Check(&result, "system initilization");

        if result == Status :: SUCCESS {
            result = System :: Start();
            System :: Check(&result, "Starting Operating system");
        } else {
            println!("FATAL ERROR: System initilization failed.");
        }
    }
}


mod BootSequence {
    use crate :: Status;

    pub fn Start() -> Status {
        println!("[1.0000] Starting boot sequence");
        Status :: SUCCESS
    }
}


mod System {
    use crate :: Status;
    use crate :: Path;
    use crate :: popen;

    pub fn Init() -> Status {
        println!("[2.0000] Initializing system");

        if !Path::new("./root").exists() {
            println!("./root not found: making it for you ...");
            popen("mkdir", &["root"]);
        }

        Status :: SUCCESS
    }

    pub fn Start() -> Status {
        // start the operating system from hardrive

        Status :: SUCCESS
    }

    pub fn Check(status: &Status, tag: &str) {
        match status {
            Status :: EXIT      =>  println!("|\t(Normal) Exited normaly [{}]", tag),
            Status :: SUCCESS   =>  println!("|\t(success) status success [{}]", tag),
            Status :: FAIL      =>  println!("|\t[FAILED] FAIL SIGNAL FROM [{}]", tag),
            Status :: FOUND     =>  println!("|\t(normal) status FOUND [{}]", tag),
            Status :: NOT_FOUND =>  println!("|\t(NOT_FOUND) status NOT_FOUND [{}]", tag),
            Status :: UNDEFINED =>  println!("|\t(UNDEFINED) status UNDEFINED [{}]", tag),
            Status :: DEFINED   =>  println!("|\t(Normal) status DEFINED [{}]", tag),
        }
    }
}

#[derive(PartialEq)]
pub enum Status {
    EXIT = 0x00,
    SUCCESS,     // 0x01
    FAIL,        // 0x02
    FOUND,       // 0x03
    NOT_FOUND,   // 0x04
    UNDEFINED,   // 0x05
    DEFINED,     // 0x06
}


pub enum UserMode {
    USER_NORM,
    USER_ROOT,
}


pub struct ConsoleInfo<'a> {
    EXIT_STATUS: &'a mut i32,
    USER_MODE: &'a mut UserMode,
}
