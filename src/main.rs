mod engine;

use std::io::BufRead;

pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), DynError> {
    matches_regex_in_file(".*", "example.txt")?;
    Ok(())
}

/// Determines whether the given regular expression matches any line in the file.
///
/// # Arguments
///
/// * `regex`: A string representing the regular expression pattern to match.
/// * `file_name`: The name of the file to process.
///
/// # Returns
///
/// * `Ok(())` if the regular expression matches at least one line in the file.
/// * `Err(DynError)` if the file does not exist or is not accessible.
///
/// Note: This method assumes that the file exists and is accessible.
fn matches_regex_in_file(regex: &str, file_name: &str) -> Result<(), DynError> {
    let file = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if engine::is_match(regex, &line)? {
            println!("{}", line);
        }
    }
    Ok(())
}

