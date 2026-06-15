use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IocEntry {
    pub hash: String,
    pub label: String,
}

pub fn is_valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn load_iocs(path: &Path) -> Result<(Vec<IocEntry>, usize), io::Error> {
    let content = fs::read_to_string(path)?;

    let mut iocs = Vec::new();
    let mut invalid_count = 0;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ',').collect();

        if parts.len() != 2 {
            invalid_count += 1;
            continue;
        }

        let hash = parts[0].trim();
        let label = parts[1].trim();

        if is_valid_sha256(hash) {
            iocs.push(IocEntry {
                hash: hash.to_string(),
                label: label.to_string(),
            });
        } else {
            invalid_count += 1;
        }
    }

    Ok((iocs, invalid_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sha256() {
        assert!(is_valid_sha256(
            "44ea92bec1f9e8aa690d8aceddf1294e9fb4a71d39769d6f383e3915ac76bb3b"
        ));
    }

    #[test]
    fn test_invalid_sha256() {
        assert!(!is_valid_sha256("invalid_hash"));
    }
}
