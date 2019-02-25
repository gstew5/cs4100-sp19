use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

//Create, write, then read a binary file.
//To display the resulting file test.o, do:
// od --endian big -A x -x test.o
fn main() -> std::io::Result<()> {
    let bytes: Vec<u8> = vec![104, 101, 108, 108, 111, 33];

    //Create file
    let mut file = OpenOptions::new()
        .read(true).write(true).open("test.o")?;

    //Write bytes
    file.write_all(&bytes[..])?;

    //Seek back to beginning of file
    file.seek(SeekFrom::Start(0))?;

    //Read file into buf
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    //Print out each byte
    for c in buf {
        println!("{:x}", c);
    };

    Ok(())
}
