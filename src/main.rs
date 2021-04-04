use core::panic;
use std::{process::Command, time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};


fn main() {
    let scan = System::new_all();
    let gta_processes = scan.get_process_by_name("GTA5.exe");
    let mut pid: usize = 0;

    // There should be only one GTA5 running
    if gta_processes.len() == 0 {
        panic!("Cannot find any running GTA5 processes. You must start GTA5 and go to a public lobby.");
    } else if gta_processes.len() >= 1 {
        pid = gta_processes[0].pid();
    }

    let gta_v = match remoteprocess::Process::new(pid as u32) {
        Ok(x) => x,
        Err(_) => {
            println!("Error: Cannot attach to the GTA5 process.\nTry again or report the bug here: https://github.com/Oscuro87/gtao-solo-lobby/issues/new.");
            let _ = Command::new("pause").status();
            std::process::exit(1);
        }
    };

    println!("Process GTA5.exe opened.");
    println!("Process GTA5.exe suspended for 10 seconds... (Game will freeze for 10 secs)");
    
    let _lock = gta_v.lock().expect("Cannot lock (suspend) GTA5.exe!");
    // We are controlling GTA5's process from here.
    let duration: Duration = Duration::from_secs(1);
    for i in 0..10 {
        std::thread::sleep(duration);
        print!("{}... ", 9 - i);
    }
    // Lock is autoreleased here as the lock is being destroyed.
}
