use nizctl::keyboard;

fn main() {
    let kbd = keyboard::Keyboard::open().unwrap();
    kbd.print_version();
    kbd.print_mapping();
    kbd.print_counter();
}
