//
// main.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_event {
    pub _click: u8,
    pub x: i8,
    pub y: i8,
}

use std::fmt;
impl fmt::Debug for input_event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{dx = {}, dy = {}, left = {}, middle = {}, right = {}}}",
            self.x,
            self.y,
            self._click & 1,
            (self._click >> 2) & 1,
            (self._click >> 1) & 1,
        )
    }
}

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main() -> io::Result<()> {
    let p = Command::new("sh")
        .arg("-c")
        .arg("xdotool getmouselocation --shell | head -n2 | egrep -o [0-9]*")
        .output()?;
    let s: Vec<_> = String::from_utf8(p.stdout)
        .unwrap()
        .split('\n')
        .take(2)
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect();
    let mut x = s[0];
    let mut y = s[1];
    let mut f = File::open("/dev/input/mice")?;
    let mut buf = [0; std::mem::size_of::<input_event>()];
    let mut ie: input_event;
    loop {
        f.read(&mut buf)?;
        unsafe {
            ie = std::mem::transmute(buf);
        }
        x += ie.x as i32;
        y += ie.y as i32;
        println!("x = {}, y = {}, mouseevent = {:?}", x, y, ie);
    }
    Ok(())
}
