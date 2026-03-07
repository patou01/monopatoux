use std::io::{self};
use std::time::Instant;
mod printing;
use printing::{print_duration, print_inspection_summary};
mod fileinfo;
use fileinfo::{collect_file_paths, collect_files_with_info_parallel};

/// Holds the result of a duplicate inspection.
pub struct InspectionResult {
    pub total_files: usize,
    pub total_size: u64,
    pub duplicate_count: usize,
    pub duplicate_files: Vec<Vec<String>>,
}

/// Inspects the given path for files and finds duplicates.
/// Returns an InspectionResult with all relevant data.
fn inspect_duplicates(path: &str) -> io::Result<InspectionResult> {
    let mut file_paths = Vec::new();
    collect_file_paths(path, &mut file_paths)?;
    let files = collect_files_with_info_parallel(&file_paths)?;
    let total_files = files.len();
    let total_size: u64 = files.values().map(|info| info.size).sum();

    // Find duplicates using compare_files
    let mut duplicate_files: Vec<Vec<String>> = Vec::new();
    let mut duplicate_count = 0;
    let mut visited = std::collections::HashSet::new();
    let file_list: Vec<_> = files.iter().collect();
    for i in 0..file_list.len() {
        let (file1_name, file1_info) = file_list[i];
        if visited.contains(file1_name) {
            continue;
        }
        let mut group = vec![file1_name.clone()];
        for (file2_name, file2_info) in file_list.iter().skip(i + 1) {
            if visited.contains(&**file2_name) {
                continue;
            }
            if fileinfo::FileInfo::compare_files(file1_info, file2_info) {
                group.push(file2_name.to_string());
                visited.insert(file2_name.to_string());
            }
        }
        if group.len() > 1 {
            duplicate_count += group.len();
            for name in &group {
                visited.insert(name.clone());
            }
            duplicate_files.push(group);
        }
    }
    Ok(InspectionResult {
        total_files,
        total_size,
        duplicate_count,
        duplicate_files,
    })
}

/// Entry point for the dupes_finder application. Parses command line arguments and runs the appropriate command.
fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let mut args = std::env::args();
    let _exe = args.next(); // skip executable name
    let result = match (args.next().as_deref(), args.next()) {
        // Accept both relative and absolute paths for inspection
        (Some("inspect"), Some(path)) => {
            let inspection = inspect_duplicates(&path)?;
            print_inspection_summary(&inspection);
            Ok(())
        },
        _ => {
            eprintln!(
                "Usage: dupes_finder inspect <path>\n<path> can be relative or absolute, e.g. ./mydir or E:/fit"
            );
            std::process::exit(1);
        }
    };
    let duration = start_time.elapsed();
    print_duration(duration);
    result
}
