//! Integration tests for dupes_finder CLI and duplicate detection

use assert_cmd::Command;
use predicates::str::contains;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

/// Test that the CLI prints usage on missing arguments
#[test]
fn test_cli_usage() {
    let mut cmd = Command::cargo_bin("dupes_finder").unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Usage: dupes_finder inspect <path>"));
}

/// Test that the CLI works with a valid path and finds no duplicates in a single-file directory
#[test]
fn test_single_file_no_duplicates() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("file1.txt");
    File::create(&file_path).unwrap().write_all(b"hello world").unwrap();
    let mut cmd = Command::cargo_bin("dupes_finder").unwrap();
    cmd.args(["inspect", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Total files found: 1"))
        .stdout(contains("Number of duplicate files: 0"));
}

/// Test that duplicate files are detected
#[test]
fn test_duplicate_files_found() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    File::create(&file1).unwrap().write_all(b"same content").unwrap();
    File::create(&file2).unwrap().write_all(b"same content").unwrap();
    let mut cmd = Command::cargo_bin("dupes_finder").unwrap();
    cmd.args(["inspect", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Total files found: 2"))
        .stdout(contains("Number of duplicate files: 2"))
        .stdout(contains("Duplicated files:"));
}

/// Test that different files are not marked as duplicates
#[test]
fn test_different_files_not_duplicates() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    File::create(&file1).unwrap().write_all(b"content one").unwrap();
    File::create(&file2).unwrap().write_all(b"content two").unwrap();
    let mut cmd = Command::cargo_bin("dupes_finder").unwrap();
    cmd.args(["inspect", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Total files found: 2"))
        .stdout(contains("Number of duplicate files: 0"));
}

/// Test with an empty directory
#[test]
fn test_empty_directory() {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("dupes_finder").unwrap();
    cmd.args(["inspect", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Total files found: 0"));
}

/// Test with nested directories and duplicates
#[test]
fn test_nested_directories_duplicates() {
    let dir = tempdir().unwrap();
    let subdir = dir.path().join("sub");
    fs::create_dir(&subdir).unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = subdir.join("b.txt");
    File::create(&file1).unwrap().write_all(b"nested content").unwrap();
    File::create(&file2).unwrap().write_all(b"nested content").unwrap();
    let mut cmd = Command::cargo_bin("dupes_finder").unwrap();
    cmd.args(["inspect", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Total files found: 2"))
        .stdout(contains("Number of duplicate files: 2"));
}

