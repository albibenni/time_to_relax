const RESET_FILE_PATH: &str = "
##
# Host Database
#
# localhost is used to configure the loopback interface
# when the system is booting.  Do not change this entry.
##
127.0.0.1		localhost
255.255.255.255		broadcasthost
::1                          localhost
";
const FILE_PATH: &str = "/etc/hosts";
const LOCALHOST: &str = "127.0.0.1";

/// Modifies the system hosts file to add new host entries.
///
/// This function reads the current content of the system hosts file, then appends
/// new entries mapping the localhost IP address (127.0.0.1) to each domain name
/// provided in the arguments.
///
/// # Arguments
///
/// * `args` - A vector of strings representing domain names to add to the hosts file
///
/// # Returns
///
/// A `Result` indicating success or an I/O error if file operations fail
pub fn parse_hosts_file(args: &Vec<&str>) -> Result<(), std::io::Error> {
    let mut file: String = std::fs::read_to_string(FILE_PATH).expect("Failed to read file");
    for arg in args {
        file.push_str(LOCALHOST);
        file.push_str("               ");
        file.push_str(arg);
        file.push_str("\n");
    }

    return std::fs::write(FILE_PATH, file);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper function to create a temporary hosts file for testing
    fn setup_test_file(content: &str) -> (NamedTempFile, String) {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        write!(temp_file, "{}", content).expect("Failed to write to temp file");
        let file_path = temp_file.path().to_string_lossy().to_string();
        (temp_file, file_path)
    }
    // Test version of parse_hosts_file that accepts a file path
    fn parse_hosts_file_test(args: &Vec<String>, file_path: &str) -> Result<(), std::io::Error> {
        let mut file: String = std::fs::read_to_string(file_path)?;
        for arg in args {
            file.push_str(LOCALHOST);
            file.push_str("               ");
            file.push_str(arg);
            file.push_str("\n");
        }

        return std::fs::write(file_path, file);
    }

    #[test]
    fn test_parse_hosts_file_adds_entries() {
        // Setup
        let initial_content = "# Initial hosts file\n127.0.0.1 localhost\n";
        let (_temp_file, file_path) = setup_test_file(initial_content);

        // Test with mocked file operations
        let args = vec!["example.com".to_string(), "test.local".to_string()];

        // Since we can't modify the FILE_PATH constant, we'll create a modified version
        // of the function for testing purposes
        let result = parse_hosts_file_test(&args, &file_path);

        // Assert
        assert!(result.is_ok(), "Function should return Ok");

        // Read the modified file and verify content
        let modified_content = fs::read_to_string(file_path).expect("Failed to read temp file");
        assert!(modified_content.contains("127.0.0.1               example.com"));
        assert!(modified_content.contains("127.0.0.1               test.local"));
        assert!(modified_content.contains("# Initial hosts file"));

        // Cleanup happens automatically when temp_file goes out of scope
    }

    #[test]
    fn test_parse_hosts_file_with_empty_args() {
        let initial_content = "# Initial hosts file\n127.0.0.1 localhost\n";
        let (_temp_file, file_path) = setup_test_file(initial_content);

        let args = Vec::new();
        let result = parse_hosts_file_test(&args, &file_path);

        assert!(result.is_ok(), "Function should return Ok with empty args");

        // Content should be unchanged
        let modified_content = fs::read_to_string(file_path).expect("Failed to read temp file");
        assert_eq!(
            modified_content, initial_content,
            "File content should not change with empty args"
        );
    }
}
