use std::cmp::min;
use std::fs::{File, Metadata};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write, BufRead};

use color_eyre::eyre::Result;

pub trait FileBufferTrait:  Read + Seek + Sized { // add write (iterator ?)
    fn open(path: &str) -> io::Result<Self>; 
    fn get_filename(&self) -> &str;
    fn previous_line<S>(&mut self, max_line_size: S) where S: Into<usize>;
    fn next_line(&mut self) -> String;
    fn read_lines<N>(&mut self, ammount: N) -> Vec<String> where N: Into<usize>;
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

    fn previous_line<S>(&mut self, max_line_size: S) where S: Into<usize> {
        let curr_pos = self.stream_position().expect("keeping stream position");
        let line_size = min((curr_pos) as usize, max_line_size.into());

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(line_size, 0);

        let line_size: i64 = line_size.try_into().expect("line_size shouldn't be > max u32");

        let pos = self.seek(SeekFrom::Current(-line_size)).expect("Seeking 0 or later"); 

        let _ = self.read_exact(&mut buffer);
        // If it breaks here, I assume it is because utf8 characters may use more than one byte
        let text = String::from_utf8(buffer).unwrap();

        let prev_line_size: i64 = text.lines().rev().next().map_or(0, |line| line.len().try_into().expect("lines shouldn't be > max u32")) +1;
        let _ = self.seek(SeekFrom::Current(-prev_line_size));
    }

    fn next_line(&mut self) -> String {
        let mut content = vec![];
        let _ = self.file_buffer.read_until(b'\n', &mut content);
        String::from_utf8(content).unwrap()
    }

    fn read_lines<N>(&mut self, ammount: N) -> Vec<String> where N: Into<usize> {
        let curr_pos = self.stream_position().unwrap();
        let data = (0..ammount.into()).map(|_| self.next_line()).collect::<Vec::<String>>();
        let _ = self.seek(SeekFrom::Start(curr_pos));
        data
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
