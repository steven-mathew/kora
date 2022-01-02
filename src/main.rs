use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == b'q' & 0b0001_1111 {
                    break;
                }
            }
            Err(err) => panic!("the world exploded: {}", err),
        }
    }
}
