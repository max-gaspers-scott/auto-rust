pub enum Lang {
    Rust,
    Python,
}

pub struct CodeFile {
    pub path: std::path::PathBuf,
    pub kind: Lang,
}

impl CodeFile {
    pub fn to_string(&self) -> String {
        "text".to_string()
        // self.path.to_string_lossy()
    }
    pub fn lint(&self) -> Result<(), std::io::Error> {
        match self.kind {
            Lang::Rust => self.lint_rust(),
            Lang::Python => self.lint_python(),
        }
    }
    fn lint_rust(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn lint_python(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
