use std::collections::HashMap;
use std::fs::File;
use std::os::unix::prelude::AsRawFd;
use std::process::{exit, Child, Command, Stdio};

// pub enum AutoRestart {
//     NEVER,
//     UNEXPECTED,
//     ALWAYS,
// }

pub struct Program {
    pub name: String,
    pub cmd: String,
    pub stdout_path: Option<String>,
    pub stderr_path: Option<String>,
    stdout_fd: i32,
    stderr_fd: i32,
    pid: u32,
    // numprocs: u8,
    // umask: u8,
    // workingdir: Option<String>,
    // autostart: bool,
    // autorestart: AutoRestart,
    // exitcodes: Vec<i8>,
    // startretries: u8,
    // starttime: u8,
    // stoptime: u8,
    // stopsignal: Vec<u8>,
    // env: HashMap<String, Option<String>>,
}

impl Program {
    pub fn new(section: String, program_cfg: HashMap<String, Option<String>>) -> Program {
        let cmd = get_cmd(&section, &program_cfg["cmd"]);

        Program {
            name: section,
            cmd: cmd,
            stdout_path: program_cfg["stdout"].clone(),
            stderr_path: program_cfg["stderr"].clone(),
            stdout_fd: -1,
            stderr_fd: -1,
            pid: 0,
        }
    }

    pub fn start(&mut self) {
        println!("starting {}", self.name);
        let mut child = Command::new(self.cmd.clone());

        if (&self.stdout_path).is_some() {
            let path = self.stdout_path.clone().unwrap();
            if path.eq("none") {
                child.stdout(Stdio::null());
            } else {
                let file = open_file(path);
                self.stdout_fd = file.as_raw_fd();
                child.stdout(file);
            }
        }
        if (&self.stderr_path).is_some() {
            let path = self.stderr_path.clone().unwrap();
            if path.eq("none") {
                child.stdout(Stdio::null());
            } else {
                let file = open_file(path);
                self.stderr_fd = file.as_raw_fd();
                child.stderr(file);
            }
        }
        let child = child.spawn().expect("Failed to spawn child process");
        self.pid = child.id();
        println!("{} is started", self.name);
    }
}

fn get_cmd(section: &str, cmd: &Option<String>) -> String {
    let cmd = cmd.clone();
    if cmd.is_none() || cmd.clone().unwrap().eq("") {
        println!("Config: cmd is not given for `{}`", section);
        exit(1);
    }
    cmd.unwrap()
}

fn open_file(path: String) -> File {
    match File::create(&path) {
        Err(e) => {
            println!("Config Error:");
            println!("=> `{}`: {}", path, e);
            exit(e.raw_os_error().unwrap());
        }
        Ok(file) => return file,
    }
}
