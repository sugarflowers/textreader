use binaryfile::BinaryReader;
use sjis::{decode, is_sjis};
use std::io::{self};

pub struct TextReader {
    pub reader: BinaryReader,
}

impl TextReader {
    pub fn open(filename: &str) -> Result<Self, io::Error> {
        Ok(Self {
            reader : BinaryReader::open(filename)?,
        })
    }
}

impl Iterator for TextReader {
    type Item = Result<String, io::Error>; 
    fn next( &mut self ) -> Option<Self::Item> {
        match self.reader.next() {
            Some(Ok(line)) => {
                if is_sjis(&line) {
                    Some(Ok(decode(line)))
                } else {
                    Some(Ok(String::from_utf8(line).unwrap()))
                }
            }
            Some(Err(e)) => Some(Err(e)),
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

