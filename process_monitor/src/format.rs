const MEGABYTE: f64 = 1000.0;

pub fn get_memory_string(memory: u64) -> String {
    let num_mbs: f64 = memory as f64 / MEGABYTE;

    if num_mbs >= 1.0 {
        return format!("{:.1}mB", num_mbs);
    }

    return format!("{}kB", memory);
}

pub fn get_cpu_string(cpu: &f32) -> String {
    return format!("{}% CPU", cpu);
}
