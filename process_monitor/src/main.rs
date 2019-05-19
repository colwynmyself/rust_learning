mod application;
mod format;
mod process;
mod system;
mod ui;

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use crate::application::{create_applications_from_processes, Application};
use crate::format::{get_cpu_string, get_memory_string};
use crate::process::Process;
use crate::system::{get_processes, get_system};
use crate::ui::{create_terminal, draw_frame};

fn main() {
    let mut system = get_system();
    let mut terminal = match create_terminal() {
        Ok(t) => t,
        Err(e) => panic!("Error creating terminal {}", e),
    };

    match terminal.clear() {
        Ok(_) => {}
        Err(e) => panic!("Error clearing terminal {}", e),
    };

    loop {
        let processes: Vec<Process> = get_processes(&mut system);
        let applications: HashMap<String, Application> =
            create_applications_from_processes(&processes);

        let mut process_strings: Vec<String> = Vec::new();
        for process in processes.iter() {
            let name = process.get_name();
            let memory_string = get_memory_string(process.memory);
            let cpu_string = get_cpu_string(&process.cpu_usage);

            process_strings.push(format!(
                "{} {} ({} / {})",
                process.pid, name, memory_string, cpu_string
            ));
        }

        let mut application_strings: Vec<String> = Vec::new();
        for (_, application) in applications.iter() {
            let name = application.name.to_string();
            let memory_string = get_memory_string(application.get_memory());
            let cpu_string = get_cpu_string(&application.get_cpu());
            application_strings.push(format!("{} - ({} / {})", name, memory_string, cpu_string));
        }

        match draw_frame(&mut terminal, process_strings, application_strings) {
            Ok(_) => {}
            Err(e) => println!("Error rendering frame {}", e),
        };

        let sleep_time = Duration::from_millis(500);
        sleep(sleep_time);
    }
}
