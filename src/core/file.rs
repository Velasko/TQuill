use std::cmp::min;
use std::fs::{File, Metadata};
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::rc::Rc;

use crate::core::diff::Diff;

#[cfg_attr(test, derive(Debug))]
enum CursorLocation {
    InDiff,
    InFile,
}

#[cfg_attr(test, derive(Debug))]
struct CursorInfo {
    diff_index: usize,
    diff_pos: usize,
    file_pos: usize,
    offset: i128,
    resting_at: CursorLocation
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
                    diff_index: 0,
                    diff_pos: 0,
                    file_pos: 0,
                    offset: 0,
                    resting_at: CursorLocation::InFile,
                },
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
                let mut offset: i128 = 0;
                for (n, diff) in self.content_diff.iter().enumerate() {
                    let slice = diff.get_slice();

                    // If before the end of that slice, return
                    if i128::from(index) < offset + (slice.start + diff.get_repl().len()) as i128  {
                        // if in file, before diff
                        return if i128::from(index) < offset + slice.start as i128 {
                            let curr_pos = ((index as i128) - offset) as usize;
                            // set new file buffer position
                            self.file_buffer.seek(SeekFrom::Start(curr_pos as u64)).map(|_| {
                                // If ok, set pos and return.
                                self.cursor_pos = CursorInfo {
                                    diff_index: n,
                                    diff_pos: 0,
                                    file_pos: curr_pos,
                                    offset: offset,
                                    resting_at: CursorLocation::InFile,
                                };
                                index
                            })
                        } else { // if in diff
                            // set new file bffer position
                            self.file_buffer.seek(SeekFrom::Start(slice.start as u64)).map(|_| {
                                // If ok, set pos and return.
                                let position = ((index as usize - slice.start) as i128 - offset) as usize;
                                self.cursor_pos = CursorInfo {
                                    diff_index: n,
                                    diff_pos: position,
                                    file_pos: slice.start,
                                    offset: offset,
                                    resting_at: CursorLocation::InDiff,
                                };
                                index
                            })
                        }

                    }

                    offset += diff.get_size();
                }

                self.file_buffer.seek(SeekFrom::Start(index)).map(|_| {
                    let bfmax = self.file_metadata.len();
                    let index = min(bfmax, (index as i128 - offset) as u64) as usize;

                    self.cursor_pos = CursorInfo {
                        diff_index: self.content_diff.len(),
                        diff_pos: 0,
                        file_pos: index,
                        offset: offset,
                        resting_at: CursorLocation::InFile,
                    };
                    ((index as i128) + offset) as u64
                })

            },
            SeekFrom::End(index) => {
                let index = (self.file_metadata.len() as i128 + index as i128) as u64;
                let mut offset = self.content_diff.iter().fold(0, |acc, diff| acc + diff.get_size());

                for (n, diff) in self.content_diff.iter().enumerate().rev() {
                    let slice = diff.get_slice();


                    // If after the beginning of the slice, return
                    let diff_end_index = (slice.start + diff.get_repl().len()) as i128 + offset;
                    let diff_start_index = (slice.start as i128) + offset;
                    if i128::from(index) >= diff_start_index {
                        // If in diff, before the file content
                        return if i128::from(index) < diff_end_index {
                            self.file_buffer.seek(SeekFrom::End(slice.start as i64 - self.file_metadata.len() as i64)).map(|_| {
                                self.cursor_pos = CursorInfo {
                                    diff_index: n,
                                    diff_pos: (index as i128 - diff_start_index) as usize,
                                    file_pos: slice.start,
                                    offset: offset,
                                    resting_at: CursorLocation::InDiff,
                                };
                                index
                            })
                        } else {
                            self.file_buffer.seek(SeekFrom::End((index as i128 - offset) as i64)).map(|_| {
                                self.cursor_pos = CursorInfo {
                                    diff_index: n,
                                    diff_pos: 0,
                                    file_pos: (index as i128 - offset) as usize,
                                    offset: offset,
                                    resting_at: CursorLocation::InFile,
                                };
                                index
                            })
                        }
                    }

                    offset -= diff.get_size();
                }
 
                self.file_buffer.seek(SeekFrom::End((index as i128 - offset) as i64)).map(|_| {
                    self.cursor_pos = CursorInfo {
                        diff_index: 0,
                        diff_pos: 0,
                        file_pos: index as usize,
                        offset: 0,
                        resting_at: CursorLocation::InFile,
                    };
                    index
                })
            },
            SeekFrom::Current(index) => {
                // if index > 0 {
                //     match self.cursor_pos {
                //     }
                // } else {
                // }
                todo!()
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
}
