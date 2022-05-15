use std::collections::HashMap;
use std::fs::File;
use std::os::unix::prelude::AsRawFd;
use std::process::{exit, Stdio};
use std::process::{Child, Command};

pub struct Program {
    pub name: String,
    pub command: String,
    pub stdout_path: Option<String>,
    pub stdout_fd: i32,
    pub stderr_path: Option<String>,
    pub stderr_fd: i32,
    pid: u32,
}

impl Program {
    pub fn new(section: String, program_cfg: HashMap<String, Option<String>>) -> Program {
        let cmd = get_command(&section, &program_cfg["command"]);

        Program {
            name: section,
            command: cmd,
            stdout_path: program_cfg["stdout"].clone(),
            stderr_path: program_cfg["stderr"].clone(),
            stdout_fd: -1,
            stderr_fd: -1,
            pid: 0,
        }
    }

    pub fn start(&mut self) {
        println!("starting {}", self.name);
        let mut child = Command::new(self.command.clone());

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

    // pub fn kill(mut self) {}
}

fn get_command(section: &str, command: &Option<String>) -> String {
    let cmd = command.clone();
    if cmd.is_none() || cmd.clone().unwrap().eq("") {
        println!("Config: command is not given for `{}`", section);
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
