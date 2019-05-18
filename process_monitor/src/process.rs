use regex::Regex;
use sysinfo::ProcessExt;

pub struct Process {
    pub pid: i32,
    pub command: String,
    pub memory: u64,
    pub cpu_usage: f32,
}

impl Process {
    pub fn new(pid: &i32, proc_: &sysinfo::Process) -> Process {
        return Process {
            pid: *pid,
            command: proc_.name().to_string(),
            memory: proc_.memory(),
            cpu_usage: proc_.cpu_usage(),
        };
    }

    pub fn get_name(&self) -> String {
        let re = Regex::new(r"^\S+").unwrap();
        let found = re.find(&self.command);

        assert!(
            !found.is_none(),
            format!("{} failed to find process name", self.command)
        );

        return found.unwrap().as_str().to_string();
    }
}
