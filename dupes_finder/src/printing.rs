use crate::InspectionResult;

/// Formats a file size in bytes into a human-readable string (kB, MB, or GB).
///
/// # Arguments
///
/// * `size` - The size in bytes.
///
/// # Returns
///
/// A formatted string representing the size in the most appropriate unit.
pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} kB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}

/// Prints the duration in a human-friendly format: Hh Mm Ss, Mm Ss, or Ss.ms depending on the length.
///
/// # Arguments
///
/// * `duration` - The duration to print.
pub fn print_duration(duration: std::time::Duration) {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    if hours > 0 {
        println!("Duration: {}h {}m {}s", hours, minutes, seconds);
    } else if minutes > 0 {
        println!("Duration: {}m {}s", minutes, seconds);
    } else {
        println!("Duration: {}.{:03}s", seconds, millis);
    }
}

/// Prints the results of a duplicate inspection in a user-friendly format.
///
/// # Arguments
///
/// * `result` - The InspectionResult to display.
pub fn print_inspection_summary(result: &InspectionResult) {
    println!("Total files found: {}", result.total_files);
    println!(
        "Total size of analyzed files: {}",
        format_size(result.total_size)
    );
    println!("Number of duplicate files: {}", result.duplicate_count);
    println!("Duplicated files:");
    for group in &result.duplicate_files {
        println!("- Group:");
        for file in group {
            println!("    {}", file);
        }
    }
}
