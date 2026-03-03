use std::fs::OpenOptions;
use std::io;
use std::io::Write;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn write_to_disk(
    path: &std::path::Path,
    text: String,
    can_overright: bool,
) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    file.write_all(text.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
