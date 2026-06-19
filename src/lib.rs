use binaryfile::BinaryReader;
use sjis::{decode, is_sjis};
use anyhow::{Context, Result};

pub struct TextReader {
    pub reader: BinaryReader,
}

impl TextReader {
    pub fn open(filename: &str) -> Result<Self> {
        Ok(Self {
            reader: BinaryReader::open(filename)?,
        })
    }

    pub fn read(&mut self) -> Result<String> {
        let buf = self.reader.read()?;
        if is_sjis(&buf) {
            Ok(decode(&buf))
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }
}

impl Iterator for TextReader {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next().map(|res| {
            let line = res.context("failed to read a line")?;

            if is_sjis(&line) {
                Ok(decode(&line))
            } else {
                String::from_utf8(line)
                    .context("failed to decode line as UTF-8")
            }
        })
    }
}

#[test]
fn file_read_test() {
    let reader = TextReader::open("test.txt").unwrap();
    for line in reader {
        println!("{}", line.unwrap());
    }
}

#[test]
fn file_read_once() {
    let reader = TextReader::open("test.txt").unwrap().read();
    println!("{}", reader.unwrap());
}
