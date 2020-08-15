use log::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

pub struct FileWatcher {
    pos: u64,
    reader: BufReader<File>,
    finish: bool,
}

impl FileWatcher {
    pub fn register(filename: String) -> Result<FileWatcher, std::io::Error> {
        let f = match File::open(filename.clone()) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let metadata = match f.metadata() {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let mut reader = BufReader::new(f);
        let pos = metadata.len();
        reader.seek(SeekFrom::Start(pos)).unwrap();
        Ok(FileWatcher {
            pos: pos,
            reader: reader,
            finish: false,
        })
    }

    pub fn watch<F: Fn(String)>(&mut self, on_line_receive: F) {
        loop {
            let mut line = String::new();
            let resp = self.reader.read_line(&mut line);
            match resp {
                Ok(len) => {
                    if len > 0 {
                        self.pos += len as u64;
                        self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        on_line_receive(line.replace("\n", ""));
                        line.clear();
                    } else {
                        if self.finish {
                            break;
                        } else {
                            //self.reopen_if_log_rotated(callback);
                            self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        }
                    }
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
    }
}
