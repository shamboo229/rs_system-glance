use sysinfo::System;
use colored::*;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Initialize the system object
    let mut sys = System::new_all();

    println!("{}", "Starting QuickStat Monitor v0.38...".bold().cyan());
    println!("Press Ctrl+C to exit.\n");

    loop {
        // 2. Refresh Data()
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        // 3. Clear the terminal screen and move cursor to top-left
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        println!("{}", "========================================".blue());
        println!("          {}          ", "QUICKSTAT MONITOR".bold());
        println!("{}", "========================================\n".blue());

        // --- MEMORY SECTION ---
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();

        // Calculations
        let used_mb = used_mem / 1024 / 1024;
        let total_mb = total_mem / 1024 / 1024;
        let mem_percentage = (used_mem as f64 / total_mem as f64) * 100.0;

        let mem_bar = create_progress_bar(mem_percentage);

        println!("{:<12} {} / {} MB", "Memory:".green().bold(), used_mb, total_mb);
        println!("{}\n", mem_bar.green());

        // --- CPU SECTION ---
        let cpu_usage = sys.global_cpu_usage() as f64;
        let cpu_bar = create_progress_bar(cpu_usage);

        println!("{:<12} {:.2}%", "CPU Load:".yellow().bold(), cpu_usage);
        println!("{}\n", cpu_bar.yellow());

        // --- SYSTEM INFO ---
        // Accessing static system info
        println!("{:<12} {}", "OS:".white().dimmed(), System::name().unwrap_or_else(|| "Unknown".to_string()));
        println!("{:<12} {}", "Host:".white().dimmed(), System::host_name().unwrap_or_else(|| "Unknown".to_string()));

        // 4. Critical Warning Logic
        if cpu_usage > 90.0 {
            println!("\n{}", " ALERT: HIGH CPU USAGE ".on_red().white().bold());
        }

        // 5. Wait 1 second (this gives sysinfo time to calculate the next CPU delta)
        thread::sleep(Duration::from_secs(1));
    }
}

/// Generates a visual bar: [■■■■□□□□] 50.0%
fn create_progress_bar(percentage: f64) -> String {
    let total_width = 25;
    // Ensure percentage doesn't exceed 100 or drop below 0 for the bar calculation
    let safe_percentage = percentage.clamp(0.0, 100.0);
    let filled_width = (safe_percentage / 100.0 * total_width as f64).round() as usize;

    let mut bar = String::from("[");

    for i in 0..total_width {
        if i < filled_width {
            bar.push('■');
        } else {
            bar.push(' ');
        }
    }

    bar.push_str(&format!("] {:.1}%", percentage));
    bar
}
