use std::fs::File;
use std::io::{self, BufReader};

use crate::core::diff::Diff;

#[cfg_attr(test, derive(Debug))]
struct FileBuffer {
    cursor: u64,
    filename: Option<String>,
    file_buffer: Option<BufReader<File>>,
    content_diff: Vec<Diff>,
}

impl FileBuffer {
    fn new() -> Self {
        Self {
            cursor: 0,
            filename: None,
            file_buffer: None,
            content_diff: Vec::new(),
        }
    }

    fn open(path: &str) -> io::Result<Self> {
        io::Result::Ok(
            Self {
                cursor: 0,
                filename: Some(String::from(path)),
                file_buffer: Some(BufReader::new(File::open(path)?)),
                content_diff: Vec::new(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_file() {
        let file = FileBuffer::open("batata.txt");
        assert!(!file.is_ok());
    }
}
