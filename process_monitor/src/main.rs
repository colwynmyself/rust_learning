mod process;
mod system;

fn main() {
    let mut system = system::get_system();

    let processes = system::get_processes(&mut system);

    for process in processes {
        let memory_string = process.get_memory_string();
        let cpu_string = process.get_cpu_string();

        println!(
            "{} {} ({} / {})",
            process.pid, process.name, memory_string, cpu_string
        );
    }
}
