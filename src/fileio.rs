use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom};
use std::io::{Read, Write};
use std::{fs::OpenOptions, io::Error};

#[derive(Debug)]
pub enum Modes {
    Read,
    Write,
    Append,
    ReadWrite,
}
pub struct FileIO {
    file: File,
    filename: String,
    mode: Modes,
}

impl FileIO {
    pub fn new(s: &str, mode: Modes) -> Result<FileIO, Error> {
        let file = match mode {
            Modes::Read => OpenOptions::new().read(true).open(s)?,
            Modes::Append => OpenOptions::new()
                .append(true)
                .create(true) // Ensure file is created if it doesn't exist
                .open(s)?,
            Modes::Write | Modes::ReadWrite => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true) // Ensure file is created if it doesn't exist
                .open(s)?,
        };
        Ok(FileIO {
            file,
            filename: s.to_string(),
            mode,
        })
    }
    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn read(&mut self, s: &mut String) -> Result<(), Error> {
        self.file.read_to_string(s)?;
        Ok(())
    }

    pub fn write(&mut self, cn: &str) -> Result<(), Error> {
        match self.mode {
            (Modes::Write | Modes::ReadWrite) => {
                self.file.seek(SeekFrom::Start(0))?; // Move cursor to the beginning
                self.file.set_len(0)?; // Truncate the file
                let bytes = cn.as_bytes();
                let mut writer = BufWriter::new(&self.file);
                writer.write_all(bytes)?; // Write new content
                writer.flush()?; // Flush the buffer
                Ok(())
            }
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Arguments",
            )),
        }
    }

    pub fn append(&mut self, cn: &str) -> Result<(), Error> {
        match self.mode {
            Modes::Append => {
                self.file.write_all(cn.as_bytes())?;
                self.file.flush()?;
                Ok(())
            }
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Arguments",
            )),
        }
    }
}
