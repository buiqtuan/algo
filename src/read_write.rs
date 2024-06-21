use std::io::{
    BufRead, BufReader, Read, Result, Write
};

fn count_line<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);

    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    match writer.write_all(msg.as_bytes()) {
        Err(e) => return Err(e),
        _ => Ok(())
    }
}