use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::process::{exit, Stdio};
use std::process::{Child, Command};

pub struct Program {
    pub name: String,
    pub command: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    process: Option<Child>,
}

impl Program {
    pub fn new(section: String, program_cfg: HashMap<String, Option<String>>) -> Program {
        let cmd = get_command(&section, program_cfg["command"]);

        Program {
            name: section,
            command: cmd,
            stdout: program_cfg["stdout"].clone(),
            stderr: program_cfg["stderr"].clone(),
            process: None,
        }
    }

    pub fn start(mut self) {
        println!("starting {}", self.name);
        // println!("stdout: {}", self.stdout.unwrap());
        let mut child = Command::new(self.command);

        // child.stdout(self.stdout.unwrap());
        // child.stderr(self.stderr.unwrap());
        self.process = Some(child.spawn().expect("Failed to spawn child process"));
        println!("{} is started", self.name);
    }
}

fn get_command(section: &str, command: &Option<String>) -> String {
    let check_command = || -> Result<String, ()> {
        let cmd = command.unwrap();
        if cmd.eq("") {
            panic!();
        }
        return Ok(cmd.clone());
    };
    match check_command() {
        Ok(cmd) => cmd.to_string(),
        Err(e) => panic!("{}: command is not given", section),
    }
}

fn open_file(path: Option<String>) -> Option<File> {
    match path {
        Some(p) => match File::create(&p) {
            Err(e) => {
                println!("Config Error:");
                println!("=> `{}`: {}", p, e);
                exit(e.raw_os_error().unwrap());
            }
            Ok(file) => return Some(file),
        },
        None => return None,
    };
}
