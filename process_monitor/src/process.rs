use sysinfo::ProcessExt;

const MEGABYTE: f64 = 1000.0;

pub struct Process {
    pub pid: i32,
    pub name: String,
    pub memory: u64,
    pub cpu_usage: f32,
}

impl Process {
    pub fn new(pid: i32, proc_: &sysinfo::Process) -> Process {
        return Process {
            pid: pid,
            name: proc_.name().to_string(),
            memory: proc_.memory(),
            cpu_usage: proc_.cpu_usage(),
        };
    }

    pub fn get_memory_string(&self) -> String {
        let num_mbs: f64 = self.memory as f64 / MEGABYTE;

        if num_mbs >= 1.0 {
            return format!("{:.1}mB", num_mbs);
        }

        return format!("{}kB", self.memory);
    }

    pub fn get_cpu_string(&self) -> String {
        return format!("{}% CPU", self.cpu_usage);
    }
}
