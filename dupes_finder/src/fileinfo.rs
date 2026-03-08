use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use xxhash_rust::xxh3::Xxh3;

/// Stores information about a file, including its size and hash value.
pub struct FileInfo {
    pub size: u64,
    pub hash: u64,
}

impl FileInfo {
    /// Compares two FileInfo objects for equality based on their hash value.
    /// Returns true if the hashes are equal, indicating the files are likely duplicates.
    pub fn compare_files(file1: &FileInfo, file2: &FileInfo) -> bool {
        file1.hash == file2.hash
    }
}

/// Recursively collects all file paths in the given directory.
pub fn collect_file_paths<P: AsRef<Path>>(path: P, files: &mut Vec<String>) -> io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_file_paths(&path, files)?;
        } else if path.is_file() {
            files.push(path.display().to_string());
        }
    }
    Ok(())
}

/// Collects file info (size, hash) for a list of file paths in parallel.
pub fn collect_files_with_info_parallel(
    file_paths: &[String],
) -> io::Result<HashMap<String, FileInfo>> {
    let results: Vec<_> = file_paths
        .par_iter()
        .map(|path_str| {
            let path = Path::new(path_str);
            let res: io::Result<(String, FileInfo)> = (|| {
                let metadata = std::fs::metadata(path)?;
                let mut file = File::open(path)?;
                let mut hasher = Xxh3::new();
                let mut buffer = [0u8; 8192];
                loop {
                    let n = file.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                let hash = hasher.digest();
                Ok((
                    path.display().to_string(),
                    FileInfo {
                        size: metadata.len(),
                        hash,
                    },
                ))
            })();
            res
        })
        .collect();
    let mut files = HashMap::new();
    for res in results {
        let (path, info) = res?;
        files.insert(path, info);
    }
    Ok(files)
}
