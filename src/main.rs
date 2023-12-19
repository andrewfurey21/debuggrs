use std::process::Command;

use pete::{Ptracer, Restart, Tracee};

fn main() {
    let mut cmd = Command::new("./main");
    let mut ptracer = Ptracer::new();

    let _ = ptracer.spawn(cmd);

    while let Some(tracee) = ptracer.wait().unwrap() {
        let regs = tracee.registers().unwrap();
        let pc = regs.rip as u64;

        let Tracee { pid, .. } = tracee;

        println!("pid: {}", pid);

        ptracer.restart(tracee, Restart::Continue).unwrap();
    }
}
