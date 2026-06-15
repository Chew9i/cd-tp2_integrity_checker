use crate::hashing::hash_file_sha256;
use crate::ioc::IocEntry;

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanStatus {
    Clean,
    Match(String),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub path: String,
    pub sha256: Option<String>,
    pub status: ScanStatus,
}

pub fn scan_target(target: &Path, iocs: &[IocEntry]) -> Vec<ScanResult> {
    let mut results = Vec::new();

    if target.is_file() {
        scan_file(target, iocs, &mut results);
    } else if target.is_dir() {
        if let Ok(entries) = fs::read_dir(target) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() {
                    scan_file(&path, iocs, &mut results);
                }
            }
        }
    }

    results
}

fn scan_file(path: &Path, iocs: &[IocEntry], results: &mut Vec<ScanResult>) {
    match hash_file_sha256(path) {
        Ok(hash) => {
            let mut status = ScanStatus::Clean;

            for ioc in iocs {
                if ioc.hash == hash {
                    status = ScanStatus::Match(ioc.label.clone());
                    break;
                }
            }

            results.push(ScanResult {
                path: path.display().to_string(),
                sha256: Some(hash),
                status,
            });
        }

        Err(e) => {
            results.push(ScanResult {
                path: path.display().to_string(),
                sha256: None,
                status: ScanStatus::Error(e.to_string()),
            });
        }
    }
}
