use nizctl::keyboard;

fn main() {
    let kbd = keyboard::Keyboard::open().unwrap();
    kbd.print_version().unwrap();
    kbd.print_mapping().unwrap();
    kbd.print_counter().unwrap();
    kbd.write_mapping().unwrap();
}
