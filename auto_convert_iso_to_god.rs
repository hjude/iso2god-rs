use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get current directory
    let base_path = env::current_dir()?;
    
    // Find all ISO files
    let iso_files: Vec<PathBuf> = fs::read_dir(&base_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().map_or(false, |ext| ext == "iso") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    // Total ISO count for progress tracking
    let total_files = iso_files.len();
    
    // Process each ISO file
    for (index, iso_path) in iso_files.iter().enumerate() {
        // Extract filename without extension
        let filename = iso_path.file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("Unknown");
        
        println!("\nProcessing: {} ({}/{})", filename, index + 1, total_files);
        
        // Dry run to extract metadata
        let dry_run_output = Command::new("iso2god-x86_64-windows.exe")
            .args(&["--dry-run", "--trim", iso_path.to_str().unwrap(), base_path.to_str().unwrap()])
            .output()?;
        
        // Parse metadata
        let mut name = filename.to_string();
        let mut type_name = "Unknown".to_string();
        
        for line in String::from_utf8_lossy(&dry_run_output.stdout).lines() {
            if line.starts_with("Name:") {
                name = line.split(':').nth(1)
                    .map(|s| s.trim())
                    .unwrap_or(filename)
                    .to_string();
            }
            if line.starts_with("Type:") {
                type_name = line.split(':').nth(1)
                    .map(|s| s.trim())
                    .unwrap_or("Unknown")
                    .to_string();
            }
        }
        
        // Sanitize name and type (remove any problematic characters)
        let safe_name = name.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "");
        let safe_type = type_name.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "");
        
        // Create target folder path
        let target_folder = base_path.join(format!("{} {}", safe_name, safe_type));
        
        // Print parsed values
        println!("Parsed NAME: {}", safe_name);
        println!("Parsed TYPE: {}", safe_type);
        println!("Target folder: {}", target_folder.display());
        
        // Create target folder if it doesn't exist
        fs::create_dir_all(&target_folder)?;
        
        // Run conversion
        let conversion_output = Command::new("iso2god-x86_64-windows.exe")
            .args(&["--trim", iso_path.to_str().unwrap(), target_folder.to_str().unwrap()])
            .output()?;
        
        // Check conversion result
        if conversion_output.status.success() {
            println!("Conversion successful");
        } else {
            eprintln!("Conversion failed");
            eprintln!("Error output: {}", String::from_utf8_lossy(&conversion_output.stderr));
        }
    }
    
    println!("\nDone. Processed {} files.", total_files);
    
    Ok(())
}