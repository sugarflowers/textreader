use binaryfile::BinaryReader;
use sjis::{decode, is_sjis};
use anyhow::{anyhow, Result};
use std::error::Error;

pub struct TextReader {
    pub reader: BinaryReader,
}

impl TextReader {
    pub fn open(filename: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            reader : BinaryReader::open(filename).map_err(|e| anyhow!(e))?,
        })
    }
    pub fn read(&mut self) -> Result<String, Box<dyn Error>> {
        let buf = self.reader.read().map_err(|e| anyhow!(e))?;

        if is_sjis(&buf) {
            Ok(decode(buf))
        } else {
            Ok(String::from_utf8(buf).map_err(|e| anyhow!(e))?)
        }
    }
}

impl Iterator for TextReader {
    type Item = Result<String, Box<dyn Error>>; 
    fn next( &mut self ) -> Option<Self::Item> {
        match self.reader.next() {
            Some(Ok(line)) => {
                if is_sjis(&line) {
                    Some(Ok(decode(line)))
                } else {
                    Some(Ok(String::from_utf8(line).unwrap()))
                }
            }
            Some(Err(e)) => Some(Err(Box::new(e))),
            None => None,
        }
    }
}

#[test]
fn file_read_test() {
    let rd = TextReader::open("test.txt").unwrap();
    for line in rd {
        println!("{}", line.unwrap());
    }
}

#[test]
fn file_read_once() {
    let rd = TextReader::open("test.txt").unwrap().read();
    println!("{}", rd.unwrap());
}

