// Notes
// lintable should only have lint logic
// * that is immpliented sepretly for Fust and python and dokcer ...
// there should also be a struct (not trat) that impoliments to string, dispay, file ops ...
//
//
//
// there is a consept of of "rust file" or "python file"
// and they would both have the same immentation for print_path and any file ops
// but they would have difrent implimentaions for lint (pathon needs tbe linted difrent then rust)

use std::process::Command;
pub trait Lintable {
    fn lint(&self, path: &str) -> bool;
}

pub struct GenericFile<L: Lintable> {
    pub path: std::path::PathBuf,
    pub linter: L,
}

impl<L: Lintable> GenericFile<L> {
    pub fn print_path(&self) {
        println!("{}", self.path.display());
    }
    pub fn path_as_str(&self) -> &str {
        self.path.as_os_str().to_str().unwrap()
    }

    pub fn lint(&self) -> bool {
        self.linter.lint(&self.path_as_str());
        true
    }
}

pub struct RustLinter;
impl Lintable for RustLinter {
    fn lint(&self, path: &str) -> bool {
        let output = Command::new("cargo").args(["clippy", path]).status();
        match output {
            Ok(res) => {
                println!("res of clippy: \n{}", res);
            }
            Err(e) => {
                println!("error from clippy: \n{}", e);
            }
        }
        true
    }
}
pub struct PythonFile {
    pub path: std::path::PathBuf, // should be path
}

impl Lintable for PythonFile {
    fn lint(&self) -> bool {
        true
    }

    fn path_as_str(&self) -> &str {
        self.path.as_os_str().to_str().unwrap()
    }
}
