use sysinfo::System; // Bring the System tool into the scope
use colored::*; // Bring color support into scope

fn main(){
    // 1. Create a "System" object
    let mut sys = System::new_all();

    // 2. "Refresh" to get the current data
    println!("{}","--- System Snapshot ---".bold().blue());

    // 3. Fetch RAM info (values are in bytes)
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    // Calculate percentage (convert to f64 for maths operation)
    let mem_percentage = (used_mem as f64 / total_mem as f64) * 100.0;

    // Display with colors
    println!("{}: {} / {} MB ({:.2}%)", "Memory".green(), used_mem / 1024 / 1024, total_mem / 1024 / 1024, mem_percentage);

    // 5. A simple conditional (The "if" statement)
    if mem_percentage > 80.0 {
        println!("{}", "Warning! High Memory Usage!".red().bold());
    }
    else {
        println!("{}", "System is healthy".green());
    }
}
