use sysinfo::SystemExt;

use crate::process::Process;

fn should_inclue_process(process: &Process) -> bool {
    if process.name.is_empty() || process.memory == 0 {
        return false;
    }

    return true;
}

pub fn get_system() -> sysinfo::System {
    return sysinfo::System::new();
}

pub fn get_processes(system: &mut sysinfo::System) -> Vec<Process> {
    system.refresh_processes();

    let mut processes: Vec<Process> = Vec::new();

    for (pid, proc_) in system.get_process_list() {
        let process = Process::new(*pid, proc_);

        if should_inclue_process(&process) {
            processes.push(process);
        }
    }

    processes.sort_by(|a, b| a.memory.cmp(&b.memory));

    return processes;
}
