use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

struct BootConfig {
    boot_directory: PathBuf
}

fn boot(boot_config: &BootConfig) -> Result<(), Box<dyn Error>> {
    let dir = &boot_config.boot_directory;

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    } else {
        if !dir.read_dir()?.next().is_none() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Directory exists but is not empty",
            )));
        }
    }

    let gumlock_path = dir.join(".gumlock");
    fs::File::create(gumlock_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_boot_creates_directory_and_gumlock_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = BootConfig {
            boot_directory: temp_dir.path().to_path_buf(),
        };

        assert!(config.boot_directory.read_dir().unwrap().next().is_none());

        let result = boot(&config);

        assert!(result.is_ok());
        assert!(config.boot_directory.exists());
        assert!(config.boot_directory.join(".gumlock").exists());
    }

    #[test]
    fn test_boot_fails_when_directory_is_not_empty() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let dummy_file_path = temp_dir.path().join("dummy_file");
        fs::File::create(&dummy_file_path).expect("Failed to create dummy file");

        let config = BootConfig {
            boot_directory: temp_dir.path().to_path_buf(),
        };

        let result = boot(&config);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Directory exists but is not empty"
        );
    }

    #[test]
    fn test_boot_creates_gumlock_file_when_directory_is_empty() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = BootConfig {
            boot_directory: temp_dir.path().to_path_buf(),
        };

        let result = boot(&config);

        assert!(result.is_ok());
        assert!(config.boot_directory.exists());
        assert!(config.boot_directory.join(".gumlock").exists());
    }
}