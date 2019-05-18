mod application;
mod format;
mod process;
mod system;

use std::collections::HashMap;

use crate::application::{create_applications_from_processes, Application};
use crate::format::{get_cpu_string, get_memory_string};
use crate::process::Process;
use crate::system::{get_processes, get_system};

fn main() {
    let mut system = get_system();

    let processes: Vec<Process> = get_processes(&mut system);
    let applications: HashMap<String, Application> = create_applications_from_processes(&processes);

    println!("\n---- Processes ----\n");
    for process in processes.iter() {
        let name = process.get_name();
        let memory_string = get_memory_string(process.memory);
        let cpu_string = get_cpu_string(&process.cpu_usage);

        println!(
            "{} {} ({} / {})",
            process.pid, name, memory_string, cpu_string
        );
    }

    println!("\n---- Applications ----\n");
    for (_, application) in applications.iter() {
        let name = application.name.to_string();
        let memory_string = get_memory_string(application.get_memory());
        let cpu_string = get_cpu_string(&application.get_cpu());
        println!("{} - ({} / {})", name, memory_string, cpu_string);
    }
}
