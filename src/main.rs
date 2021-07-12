#![feature(duration_constants)]
use nizctl::keyboard;

fn main() {
    let kbd = keyboard::Keyboard::open().unwrap();
    println!("{}", kbd.read_version().unwrap());
    println!("{:?}", kbd.read_keymap().unwrap());
    println!("{:?}", kbd.read_counter().unwrap());
}
