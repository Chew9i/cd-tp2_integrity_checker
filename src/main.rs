mod hashing;
mod ioc;
mod report;
mod scanner;

use std::env;
use std::path::Path;

use ioc::load_iocs;
use report::write_csv_report;
use scanner::{ScanStatus, scan_target};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        println!(
            "Usage:
tp2_integrity_checker --target <FILE_OR_DIRECTORY> --ioc <IOC_FILE> --report <REPORT_FILE>"
        );
        return;
    }

    let target = Path::new(&args[2]);
    let ioc_file = Path::new(&args[4]);
    let report_file = Path::new(&args[6]);

    println!("TP2 File Integrity Checker and IOC Matcher");
    println!("Target: {}", target.display());
    println!("IOC file: {}", ioc_file.display());
    println!("Report: {}", report_file.display());

    let (iocs, invalid_iocs) = match load_iocs(ioc_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading IOC file: {}", e);
            return;
        }
    };

    let results = scan_target(target, &iocs);

    let matches: Vec<_> = results
        .iter()
        .filter(|r| matches!(r.status, ScanStatus::Match(_)))
        .collect();

    println!("\nSummary:");
    println!("* Files scanned: {}", results.len());
    println!("* IOC entries loaded: {}", iocs.len());
    println!("* Invalid IOC lines: {}", invalid_iocs);
    println!("* Matches found: {}", matches.len());
    println!("* Errors: 0");

    if !matches.is_empty() {
        println!("\nMatches:");

        for m in &matches {
            if let ScanStatus::Match(label) = &m.status {
                println!("[ALERT] {}", m.path);
                println!("SHA-256: {}", m.sha256.clone().unwrap_or_default());
                println!("IOC label: {}", label);
            }
        }
    }

    if let Err(e) = write_csv_report(&results, report_file) {
        eprintln!("Failed to write report: {}", e);
        return;
    }

    println!("\nCSV report written to {}", report_file.display());
}
