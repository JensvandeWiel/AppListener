#![windows_subsystem = "windows"]

use sysinfo::{Process, ProcessExt, System, SystemExt};
use std::{env, time};
use std::collections::HashMap;
use std::env::{Args, Vars};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::ptr::null;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use dotenvy::dotenv;


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


    let mut envr: HashMap<String, String> = HashMap::new();

    for (key, value) in env::vars() {
        envr.insert(key, value);
    }


    loop {
        if is_process_running(envr["TRIGGER_APP"].clone()) {
            let mut app = Command::new(envr["RUN_APP"].clone()).spawn().unwrap();

            while is_process_running(envr["TRIGGER_APP"].clone()) {

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
