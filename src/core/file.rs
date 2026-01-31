use std::cmp::min;
use std::fs::{File, Metadata};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

use color_eyre::eyre::Result;

pub trait FileBufferTrait:  Read + Seek + Sized { // add write (iterator ?)
    fn open(path: &str) -> io::Result<Self>; 
    fn get_filename(&self) -> &str;
    fn read(&mut self) -> String;
}

#[cfg_attr(test, derive(Debug))]
pub struct FileBuffer {
    filename: String,
    file_metadata: Metadata,
    file_buffer: Cursor<Vec<u8>>,
}

impl FileBufferTrait for FileBuffer {
    fn open(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut content = Vec::new();
        let _ = file.read_to_end(&mut content);
        io::Result::Ok(
            Self {
                filename: String::from(path),
                file_buffer: Cursor::new(content),
                file_metadata: metadata,
            }
        )
    }

    fn get_filename(&self) -> &str {
        self.filename.as_str()
    }

    fn read(&mut self) -> String {
        let mut content = String::new();
        let _ = self.file_buffer.seek(SeekFrom::Start(0));
        let _ = self.file_buffer.read_to_string(&mut content);
        content
    }
}

impl Read for FileBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file_buffer.read(buf)
    }
}

impl Seek for FileBuffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.file_buffer.seek(pos)
    }
}

impl Write for FileBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file_buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file_buffer.flush()?;

        let mut file = File::open(&self.filename)?;
        file.write(self.file_buffer.get_ref())?;
        file.flush()
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

    // Test all operations of Seek with the following variations
    //
    // Files:
    // - Empty file and empty diff
    // - Empty file and one diff
    // - Empty file and multiple diff
    // - Non-empty file and empty diff
    // - Non-empty file and one diff
    // - Non-empty file and multiple diff
    //
    // Index (do same with negative values for methods that allow):
    // - In file size
    // - In file size +(negative diff)
    // - Greater than file size but less than +diff
    // - Greater than file+diff
    // - Greater than file+(negative diff)
    //
    // Endpoint starting from/ending at:
    // - diff
    // - file
}
