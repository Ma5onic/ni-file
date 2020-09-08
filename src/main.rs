use nom::{self, bytes, IResult};
use std::io;
use cb::Offset;

mod cb;
mod offset;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/compressed-body");
    let mut stack: Vec<u8> = include_bytes!("../examples/uncompressed-header").to_vec();
    let mut rem = FILE.clone();

    loop {
        if let Ok((r, o)) = cb::get_control_bytes(rem) {
            rem = r;
            println!("{:?}", o);

            match o {
                Offset::Dictionary { length, offset } => {
                    let mut dict = offset::fetch_offset(&stack, offset, length);
                    println!("FOUND OFFSET: {:?}", dict);
                    stack.append(&mut dict);
                    // break;
                }
                Offset::Literal { length } => {
                    let (r, bytes) = take_bytes(rem, length).unwrap();
                    rem = r;

                    stack.append(&mut bytes.to_vec());
                }
            }
        } else {
            break;
        }
    }

    println!("\nstack:");
    for byte in stack.clone() {
        print!("{:02X} ", byte);
    }
    println!("\n");

    println!("text:");
    for byte in stack {
        print!("{}", byte as char);
    }
    println!("\n");

    Ok(())
}

fn take_bytes(i: &[u8], l: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(l)(i)
}
