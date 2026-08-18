use file_ops::{instert_word_left, prepend_line_to_file, remove_top_line};

use std::{
    env::current_dir,
    process::{Command, Stdio},
};

pub fn add_pub() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_names = Command::new("ls")
        .args(["backend/src/models"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(file_names.stdout).unwrap();
    let file_names: Vec<&str> = stdout.split_whitespace().collect();
    for f in file_names {
        if f == "mod.rs" {
            continue;
        }
        let new_path = current_dir().unwrap().join("backend/src/models").join(f);
        remove_top_line(&new_path)?;
        prepend_line_to_file(
            &new_path,
            "#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]",
        )?;
        instert_word_left(&new_path, 2, "pub")?;
    }
    Ok(())
}
