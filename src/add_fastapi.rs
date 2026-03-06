use fs_extra::dir::{CopyOptions, copy};
use std::path::Path;
pub fn add_fastapi(destination: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let from = Path::new("fastapi-template");
    // let destination = format!("../{}", path);
    // let to = Path::new(&destination);

    let options = CopyOptions::new(); // Use default options

    println!(
        "Attempting to copy directory from {:?} to {:?}",
        from, destination
    );

    // This function handles creating destination folders and copying all files/subdirectories

    copy(from, destination, &options).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
    println!("✅ Directory copied successfully!");
    Ok(())
}

