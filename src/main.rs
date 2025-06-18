#![windows_subsystem = "windows"]
use std::process::id;
use std::process::Command;
use std::{thread, time};
use sysinfo::{Pid, System};

use std::env;
use std::process::Stdio;
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let args: Vec<String> = env::args().collect();

    let mut target_pid = if args.len() > 1 {
        let pid = args[1].parse::<u32>().unwrap();
        let args_pid = Pid::from_u32(pid);
        args_pid
    } else {
        let parent_pid = sys.process(Pid::from_u32(id())).unwrap().parent().unwrap();
        parent_pid
    };

    let target_process = sys.process(target_pid).unwrap();
    let target_path = target_process.exe().unwrap();
    let cmdline: Vec<String> = target_process.cmd().to_vec();

    let mut sys = System::new_all();
    loop {
        sys.refresh_all();

        target_pid = match sys.process(target_pid) {
            Some(_) => target_pid,
            None => {
                let mut comm = Command::new(target_path);
                for arg in &cmdline {
                    comm.arg(arg);
                }

                comm.stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());

                #[cfg(target_os = "windows")]
                comm.creation_flags(CREATE_NO_WINDOW);

                let child= comm.spawn().unwrap();
                Pid::from_u32(child.id())
            }
        };

        let sleep_time: time::Duration = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
    }
}
