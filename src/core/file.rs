use std::fs::{File, Metadata};
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::cell::RefCell;

use crate::core::diff::Diff;

struct DiffIndex { 
    diff_pos: usize,
    index: usize
}

struct FileIndex {
    prev_diff: Option<usize>,
    index: u64,
}

#[cfg_attr(test, derive(Debug))]
enum CursorLocation {
    InDiff(DiffIndex),
    InFile(FileIndex),
}

#[cfg_attr(test, derive(Debug))]
struct CursorInfo {
    offset: i64,
    pos: RefCell<CursorLocation>
}

#[cfg_attr(test, derive(Debug))]
struct FileBuffer {
    filename: String,
    file_buffer: BufReader<File>,
    file_metadata: Metadata,
    content_diff: Vec<Diff>,
    cursor_pos: CursorInfo,
}

impl FileBuffer {
    fn open(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        io::Result::Ok(
            Self {
                filename: String::from(path),
                file_buffer: BufReader::new(file),
                file_metadata: metadata,
                cursor_pos: CursorInfo {
                    offset: 0,
                    pos: RefCell::new(CursorLocation::InFile(FileIndex { prev_diff: None, index: 0 })),
                },
                content_diff: Vec::new(),
            }
        )
    }
}

impl Read for FileBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // let og_pos = self.file_buffer.stream_position();
        let content = self.file_buffer.read(buf);
        // let new_pos = self.file_buffer.stream_position();

        content
    }
}

impl Seek for FileBuffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(index) => {
                todo!();
            },
            SeekFrom::End(index) => {
                todo!();
            },
            SeekFrom::Current(index) => {
                if index > 0 {
                    let cursor_info = self.cursor_pos.pos.get_mut();
                    match cursor_info {
                        CursorLocation::InDiff(cursor) => {
                            let index: usize = index.try_into().unwrap();
                            let cur_diff = &self.content_diff[cursor.diff_pos];
                            if cursor.index + index < cur_diff.get_slice().len() {
                                cursor.index += index;
                                let base_index: i64 = (cur_diff.get_slice().start + cursor.index).try_into().unwrap();
                                Ok((base_index + &self.cursor_pos.offset).try_into().unwrap())
                            } else {
                                let new_index = index + cursor.index - cur_diff.get_repl().len();
                                self.cursor_pos.offset += cur_diff.get_size();
                                self.cursor_pos.pos = RefCell::new(CursorLocation::InFile(FileIndex {
                                    prev_diff: Some(cursor.diff_pos),
                                    index: cur_diff.get_slice().end.try_into().unwrap()
                                }));

                                self.seek(SeekFrom::Current(new_index as i64))
                            }
                        },
                        CursorLocation::InFile(cursor) => {
                            let pos = cursor.index;
                            let mut local_offset = 0;
                            for (n, diff) in self.content_diff.iter().enumerate() {
                                if pos + index - local_offset < diff.get_slice().start {
                                    // If in file, before diff

                                } else if pos + index - local_offset < diff.get_slice().start + diff.get_repl().len() {
                                    //If in diff
                                }
                                local_offset += diff.get_size();
                            }
                            todo!()
                        }
                    }
                } else {
                    todo!("impl when going back");
                }
            },
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

        let diff = Diff::new(3..5, String::from("").into_bytes().as_slice());
        let diff2 = Diff::new(7..9, String::from("").into_bytes().as_slice());

        file.content_diff.push(diff);
        file.content_diff.push(diff2);

        // println!("{:?}", file.seek(SeekFrom::End(-2)));

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
