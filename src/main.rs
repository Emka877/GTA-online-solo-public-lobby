use core::panic;
use remoteprocess::Lock;
use std::{any::Any, time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};


fn main() {
    let scan = System::new_all();
    let gta_processes = scan.get_process_by_name("GTA5.exe");
    let mut pid: usize = 0;

    // There should be only one GTA5 running
    if gta_processes.len() == 0 {
        panic!("Cannot find any running GTA5 processes.");
    } else if gta_processes.len() >= 1 {
        pid = gta_processes[0].pid();
    }

    let gta_v = remoteprocess::Process::new(pid as u32).expect("Cannot open process gta5.exe!");
    println!("Process GTA5.exe opened.");
    let _lock = gta_v.lock().expect("Cannot lock (suspend) GTA5.exe!");
    println!("Process GTA5.exe suspended for 10 seconds... (Game will freeze for 10 secs)");
    let duration: Duration = Duration::from_secs(10);
    std::thread::sleep(duration);
    println!("Everyone should be gone now, hf");
    // Lock is autoreleased here as the lock is being destroyed.
}
