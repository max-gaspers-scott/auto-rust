use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, io};

pub fn append_to_file(path: &std::path::Path, text: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(text.as_bytes())?;
    file.flush()?;
    Ok(())
}

// still working on it
// pub fn append_or_replace(
//     path: &std::path::Path,
//     text: String,
//     name: String,
// ) -> Result<(), io::Error> {
//     // try to find function
//
//     let mut file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(path)?;
//     let contents = fs::read_to_string(path)?;
//
//     // remove it
//     // append one
//     //
//     Ok(())
// }

//pub fn remove_funciton(path: &std::path::Path, name: String) -> Result<(), io: Error> {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//
//     }
// }

pub fn prepend_line_to_file(
    path: &std::path::Path,
    line_to_add: &str,
) -> Result<(), std::io::Error> {
    let original_content = fs::read_to_string(path)?;

    let new_content = format!("{}\n{}", line_to_add, original_content);

    fs::write(path, new_content)?;

    Ok(())
}

pub fn instert_word_left(
    path: &std::path::Path,
    line: usize,
    word: &str,
) -> Result<(), std::io::Error> {
    let original_content = fs::read_to_string(path)?;
    let mut lines: Vec<String> = original_content
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    for l in line..lines.len() - 2 {
        let new_line = format!("{} {}", word, lines[l]);
        lines[l] = new_line;
    }
    let new_content = lines.join("\n");
    fs::write(path, new_content)?;
    Ok(())
}

pub fn remove_top_line(path: &std::path::Path) -> Result<(), std::io::Error> {
    let original_content = fs::read_to_string(path)?;
    let mut lines: Vec<String> = original_content
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    lines.remove(0);
    let new_content = lines.join("\n");
    fs::write(path, new_content)?;
    Ok(())
}
