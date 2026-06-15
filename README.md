# TP2 – File Integrity Checker and IOC Matcher in Rust

## Overview

This project was developed as part of the **Programming with Rust** module.

The application is a command-line tool that calculates SHA-256 hashes for files, compares them against a list of Indicators of Compromise (IOCs), and generates a CSV report summarizing the scan results.

The objective is to simulate a basic defensive security workflow while practicing secure systems programming in Rust.

---

## Features

* SHA-256 file hashing
* IOC file loading and validation
* File and directory scanning
* Detection of known suspicious hashes
* CSV report generation
* Invalid IOC handling
* Error handling using Rust `Result`
* Unit tests
* Dependency security audit using `cargo audit`

---

## Project Structure

```text
tp2_integrity_checker/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── hashing.rs
│   ├── ioc.rs
│   ├── scanner.rs
│   └── report.rs
├── samples/
│   ├── files/
│   └── iocs.txt
├── reports/
│   └── scan_report.csv
├── screenshots/
└── report/
```

## Dependencies

The project uses the following external crate:

```toml
sha2 = "0.10"
```

---

## Build

```bash
cargo build
```

---

## Run

```bash
cargo run -- --target samples/files --ioc samples/iocs.txt --report reports/scan_report.csv
```

---

## Example Output

```text
TP2 File Integrity Checker and IOC Matcher

Summary:
* Files scanned: 3
* IOC entries loaded: 2
* Invalid IOC lines: 1
* Matches found: 1
* Errors: 0

Matches:
[ALERT] samples/files/suspicious_dropper.txt

CSV report written to reports/scan_report.csv
```

---

## Testing

Run unit tests:

```bash
cargo test
```

---

## Code Quality

Format the code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy -- -D warnings
```

Run dependency audit:

```bash
cargo audit
```

---

## Security Notes

* SHA-256 is used for integrity verification.
* Invalid IOC entries do not stop program execution.
* Missing files are handled gracefully.
* No unsafe Rust code is used.

---

## Author

Student: Zeinebou Elhachmi

Module: Programming with Rust

Academic Year: 2025–2026
