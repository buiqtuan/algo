use std::io::{Result, Read};

pub struct RotDecoder<R: Read> {
    input: R,
    rot: u8
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.input.read(buf)?;
        for byte in &mut buf[..bytes_read] {
            if byte.is_ascii_alphabetic() {
                *byte = match *byte {
                    b'a'..=b'z' => (*byte - b'a' + self.rot)%26 + b'a',
                    b'A'..=b'Z' => (*byte - b'A' + self.rot)%26 + b'A',
                    _ => *byte
                }
            }
        }
        Ok(bytes_read)
    }
}