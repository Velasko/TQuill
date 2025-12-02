use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

use crate::core::diff::Diff;

#[cfg_attr(test, derive(Debug))]
struct FileBuffer {
    filename: String,
    file_buffer: BufReader<File>,
    cursor_pos: u64,
    content_diff: Vec<Diff>,
}

impl FileBuffer {
    fn open(path: &str) -> io::Result<Self> {
        io::Result::Ok(
            Self {
                filename: String::from(path),
                file_buffer: BufReader::new(File::open(path)?),
                cursor_pos: 0,
                content_diff: Vec::new(),
            }
        )
    }
}

impl Read for FileBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let og_pos = self.file_buffer.stream_position();
        let content = self.file_buffer.read(buf);
        let new_pos = self.file_buffer.stream_position();

        content
    }
}

impl Seek for FileBuffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(index) => {
                let mut acc: i128 = 0;
                let index = index as u64;
                for diff in &self.content_diff {

                    let slice = diff.get_slice();
                    let start = slice.start as i128;
                    let end = (slice.start + diff.get_repl().len()) as i128;

                    if (index as i128) <= end + acc {
                        return if (index as i128) < start + acc {
                            // if index before slice:
                            self.file_buffer.seek(SeekFrom::Start((index as i128 - acc) as u64)).map(|_| index)
                        } else {
                            // if index in slice:
                            self.file_buffer.seek(SeekFrom::Start(start as u64)).map(|_| index)
                        };
                    }

                    // acc = how much slots the slice added | removed
                    acc += diff.size_diff()
                }
            
                self.cursor_pos = index;
                self.file_buffer.seek(SeekFrom::Start(((index as i128)- acc) as u64)).map(|_| index)
            },
            SeekFrom::End(index) => {
                let seek = self.file_buffer.seek(SeekFrom::End(index));

                // sum all complete acc. reverse into decreasing
                // will that work ??
                for diff in &self.content_diff {
                }

                todo!()
            },
            SeekFrom::Current(index) => todo!(),
        }
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

    #[test]
    fn how_do_i_fkn_read() {
        let mut buffer = String::new();
        let mut file = FileBuffer::open("src/main.rs").unwrap();

        let diff = Diff::new(3..5, String::from("batata").into_bytes().as_slice());

        file.content_diff.push(diff);

        let _ = file.seek(SeekFrom::End(0));

    }
}
