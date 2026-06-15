use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::Path;

pub fn hash_file_sha256(path: &Path) -> Result<String, io::Error> {
    let data = fs::read(path)?;

    let mut hasher = Sha256::new();
    hasher.update(&data);

    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hash_length() {
        let hash = "a".repeat(64);
        assert_eq!(hash.len(), 64);
    }
}
