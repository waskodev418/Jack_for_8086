use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct Reader {
    reader: Option<BufReader<File>>,
    output_lines: Vec<String>,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            reader: None,
            output_lines: Vec::new(),
        }
    }

    pub fn input(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        self.reader = Some(BufReader::new(file));
        self.output_lines.clear();
        Ok(())
    }

    /// Reads the next raw line and CLEANS it immediately before returning.
    /// Returns None when reaching End-Of-File (EOF).
    pub fn read_line(&mut self) -> Option<String> {
        let reader = self.reader.as_mut().expect("No input file loaded!");
        let mut raw_line = String::new();

        while reader.read_line(&mut raw_line).unwrap_or(0) > 0 {
            // 1. Strip comments (everything after //)
            let without_comment = raw_line.split("//").next().unwrap_or("");

            // 2. Strip all whitespace (spaces, tabs, \r, \n)
            let clean_line: String = without_comment.trim().to_string();
              
            raw_line.clear();

            // 3. Skip completely empty lines
            if !clean_line.is_empty() {
                return Some(clean_line);
            }
        }

        None // EOF
    }

    pub fn push_output(&mut self, line: String) {
        if !line.is_empty() {
            self.output_lines.push(line);
        }
    }

    pub fn output(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for line in &self.output_lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}