#[macro_use]
extern crate ini;

mod program;

use program::Program;

fn main() {
    let config = ini!("taskmaster.conf");
    let mut programs: Vec<Program> = Vec::new();
    println!("hashmap length; {}", config.len());
    for (section, program_cfg) in config.into_iter() {
        let prg = Program::new(section, program_cfg);
        programs.push(prg);
        // if programs.
    }

    // for prg in programs.into_iter() {
    //     println!("{}", prg.to_string());
    // }
}
