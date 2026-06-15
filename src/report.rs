use crate::scanner::{ScanResult, ScanStatus};

use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path::Path;

pub fn write_csv_report(results: &[ScanResult], report_path: &Path) -> io::Result<()> {
    if let Some(parent) = report_path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(report_path)?;

    writeln!(file, "path,sha256,status,label")?;

    for result in results {
        let (status, label) = match &result.status {
            ScanStatus::Clean => ("CLEAN", String::new()),
            ScanStatus::Match(label) => ("MATCH", label.clone()),
            ScanStatus::Error(err) => ("ERROR", err.clone()),
        };

        writeln!(
            file,
            "{},{},{},{}",
            result.path,
            result.sha256.clone().unwrap_or_default(),
            status,
            label
        )?;
    }

    Ok(())
}
