#![no_std]

use libtock::console;

fn main() {
    const BUFFER_LEN: usize = 128;

    let mut buf = [0; BUFFER_LEN];
    for _ in 0..5 {
        for i in 1..buf.len() {
            for j in 0..i {
                buf[j] = b'0' + ((i % 10) as u8);
            }
            buf[i] = b'\n';
            console::write_raw(&mut buf[..(i+1)]);
        }
    }
}

