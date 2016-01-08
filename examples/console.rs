// Copyright © 2016, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
extern crate rand;
extern crate wio;
use std::mem::{swap};
use wio::console::{CharInfo, Input, ScreenBuffer};
fn main() {
    let stdin = ScreenBuffer::from_stdin().unwrap();
    let mut backbuf = ScreenBuffer::new().unwrap();
    let mut frontbuf = ScreenBuffer::new().unwrap();
    loop {
        if stdin.available_input().unwrap() > 0 {
            let input = stdin.read_input().unwrap();
            for i in input {
                if let Input::Key(x) = i {
                    if x.wVirtualKeyCode == 0x1B { return }
                }
            }
        }
        let info = backbuf.info().unwrap();
        let size = info.size();
        let buf: Vec<_> = (0..(size.0 * size.1)).map(|_| {
            let color = rand::random::<u16>() & 0xff;
            CharInfo::new(b'x' as u16, color)
        }).collect();
        backbuf.write_output(&buf, size, (0, 0)).unwrap();
        swap(&mut backbuf, &mut frontbuf);
        frontbuf.set_active().unwrap();
    }
}
