use fs_extra::dir::{CopyOptions, copy};
use std::path::Path;
pub fn add_fastapi(destination: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // bad code :(
    //TODO:  should not hardcode like this
    // maybe the auto-rust should be on a server and files can be pulled down.
    // this would also fix other hardcode path issues like in add_fastapi and gen_docker
    let from = Path::new("/home/mgs/auto-rust/fastapi-template");
    let destination = format!("{}", destination.to_string_lossy());

    let options = CopyOptions::new(); // Use default options

    println!(
        "Attempting to copy directory from {:?} to {:?}",
        from, destination
    );

    // This function handles creating destination folders and copying all files/subdirectories
    copy(from, destination, &options)?;
    println!("✅ Directory copied successfully!");
    Ok(())
}
