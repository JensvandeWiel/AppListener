extern crate sysinfo;
extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use sysinfo::{Process, ProcessExt, System, SystemExt};
use dotenv::dotenv;
use std::{env, time};
use std::env::{Args, Vars};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::ptr::null;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    //create config if not there already
    if !Path::new(".env").exists() {
        let mut file = File::create(".env")
            .expect("Error encountered while creating file!");
        file.write_all(b"TRIGGER_APP=program.exe\nRUN_APP=tool.exe")
            .expect("Error while writing to file");
    }
    // get config ok
    dotenv().ok().expect("Error encountered while loading .env");

    loop {
        if is_process_running(dotenv!("TRIGGER_APP").to_string()) {
            let mut app = Command::new(dotenv!("RUN_APP").to_string()).spawn().unwrap();

            while is_process_running(dotenv!("TRIGGER_APP").to_string()) {

            }
            app.kill().expect("!kill");
        }
    }

}
fn is_process_running(s: String) -> bool {
    let sys = System::new_all();
    for (_, process) in sys.processes()
    {
        if process.name().contains(&s){
            return true;
        }
    }
    return false;
}
