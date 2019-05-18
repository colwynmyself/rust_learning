use std::collections::HashMap;

use crate::process::Process;

// Essentially a collection of processes with the same name
pub struct Application<'a> {
    pub name: String,
    pub processes: HashMap<i32, &'a Process>,
}

impl<'a> Application<'a> {
    pub fn new(name: String) -> Application<'a> {
        return Application {
            name: name.to_string(),
            processes: HashMap::new(),
        };
    }

    pub fn add_process(&mut self, process: &'a Process) {
        assert_eq!(self.name, process.get_name());

        self.processes.insert(process.pid, process);
    }

    pub fn get_memory(&self) -> u64 {
        let mut memory: u64 = 0;

        for (_, process) in self.processes.iter() {
            memory += process.memory;
        }

        return memory;
    }

    pub fn get_cpu(&self) -> f32 {
        let mut cpu: f32 = 0.0;

        for (_, process) in self.processes.iter() {
            cpu += process.cpu_usage;
        }

        return cpu;
    }
}

pub fn create_applications_from_processes(
    processes: &Vec<Process>,
) -> HashMap<String, Application> {
    let mut applications: HashMap<String, Application> = HashMap::new();

    for process in processes {
        let name = process.get_name();

        if applications.get(&name).is_none() {
            applications.insert(name.to_string(), Application::new(name.to_string()));
        }

        let application: &mut Application = applications.get_mut(&name).unwrap();

        application.add_process(&process);
    }

    return applications;
}
