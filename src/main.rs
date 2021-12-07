use std::{time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};
use std::io::{stdin, stdout, Read, Write};

enum Errors {
    NoProcess = 1,
    CantAttach = 2,
}

impl Into<i32> for Errors {
    fn into(self) -> i32 {
        self as i32
    }
}


fn pause() {
    let mut stdout = stdout();
    let mut empty_buffer: &mut [u8] = &mut [0];
    stdout.write("Press any key...".as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut empty_buffer).unwrap(); // Blocking read
}

fn main() {
    let scan = System::new_all();
    let gta_processes = scan.get_process_by_name("GTA5.exe");
    let mut pid: usize = 0;

    // There should be only one GTA5 running
    if gta_processes.len() == 0 {
        println!("Cannot find any running GTA5 processes. You must start GTA5 and go to a public lobby.");
        pause();
        std::process::exit(Errors::NoProcess.into());
    } else if gta_processes.len() >= 1 {
        pid = gta_processes[0].pid();
    }

    let gta_v = match remoteprocess::Process::new(pid as u32) {
        Ok(x) => x,
        Err(_) => {
            println!("Error: Cannot attach to the GTA5 process.\nTry again or report the bug here: https://github.com/Oscuro87/gtao-solo-lobby/issues/new.");
            pause();
            std::process::exit(Errors::CantAttach.into());
        }
    };

    println!("Process GTA5.exe opened.");
    println!("DO NOT CLOSE THIS WINDOW  OR YOUR GAME WILL HANG FOREVER.");
    println!("Process GTA5.exe suspended for 10 seconds... (Game will freeze for 10 secs)");

    let _lock = gta_v.lock().expect("Cannot lock (suspend) GTA5.exe!");
    // We are controlling GTA5's process from here.
    let duration: Duration = Duration::from_millis(250);
    for i in 0..10 {
        print!("{}... ", 10 - i);
        stdout().flush().unwrap();
        std::thread::sleep(duration);
    }
    // Lock is auto released here as the lock is being destroyed.
}
