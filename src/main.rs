#![windows_subsystem = "windows"]
use std::{thread, time};
use std::process::id; 
use std::process::Command;
use sysinfo::{System, Pid};

use std::env;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let args: Vec<String> = env::args().collect();

    let mut target_pid =  if args.len() > 1 {
        let pid = args[1].parse::<u32>().unwrap();
        let args_pid = Pid::from_u32(pid);
        args_pid

    }else {
        let parent_pid = sys.process(Pid::from_u32(id())).unwrap().parent().unwrap();
        parent_pid
    };
    
    let target_process= sys.process(target_pid).unwrap();
    let target_path= target_process.exe().unwrap();

    let mut sys = System::new_all();
loop {
    sys.refresh_all();

    target_pid = match sys.process(target_pid) {
        Some(_) => target_pid,
        None    => {
            let spawn = Command::new(target_path).spawn().unwrap();
            Pid::from_u32(spawn.id())  
        }
    };

    let sleep_time: time::Duration = time::Duration::from_millis(1000);
    thread::sleep(sleep_time);

    }
}
