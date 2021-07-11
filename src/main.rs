#![feature(duration_constants)]
use nizctl::keyboard;

fn main() {
    let kbd = keyboard::Keyboard::open().unwrap();

    kbd.print_version().unwrap();
    kbd.print_mapping().unwrap();
    kbd.print_counter().unwrap();
    kbd.keyunlock().unwrap();
    kbd.calib().unwrap();
    println!("start calib");
    std::thread::sleep(std::time::Duration::SECOND * 5);
    kbd.calib_press().unwrap();
}
