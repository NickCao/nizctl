use clap::Clap;
use dialog::DialogBox;
use nizctl::{config, keyboard};
use std::io::Read;

#[derive(Clap, Debug)]
#[clap(name = "nizctl")]
struct Nizctl {
    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Pull(Pull),
    Push(Push),
    Lock(Lock),
    Unlock(Unlock),
    Calib(Calib),
}

#[derive(Clap, Debug)]
struct Pull {}

#[derive(Clap, Debug)]
struct Push {}

#[derive(Clap, Debug)]
struct Lock {}

#[derive(Clap, Debug)]
struct Unlock {}

#[derive(Clap, Debug)]
struct Calib {}

fn main() {
    let opts: Nizctl = Nizctl::parse();
    match opts.sub {
        SubCommand::Pull(_) => {
            let kbd = keyboard::Keyboard::open().unwrap();
            println!(
                "{}",
                config::Keymap::new(format!("niz/{}", kbd.name), kbd.read_keymap().unwrap())
                    .encode()
                    .unwrap()
            );
        }
        SubCommand::Push(_) => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            keyboard::Keyboard::open()
                .unwrap()
                .write_keymap(config::keymap_from_layers(
                    config::Keymap::decode(&buffer).unwrap().layers,
                ))
                .unwrap()
        }
        SubCommand::Lock(_) => {
            if dialog::Question::new("do you really want to lock your keyboard, you will need another keyboard to unlock").title("Warning").show().unwrap() == dialog::Choice::Yes
            {
                keyboard::Keyboard::open().unwrap().keylock().unwrap();
            }
        }
        SubCommand::Unlock(_) => {
            keyboard::Keyboard::open().unwrap().keyunlock().unwrap();
        }
        SubCommand::Calib(_) => {
            let ans = dialog::Question::new("Before starting the calibration process, make sure that all the keys are released, if you are seeing this message in your terminal, either install zenity or kdialog, or use another keyboard during the process.").title("Reminder").show().unwrap();
            if ans != dialog::Choice::Yes {
                return;
            }
            let kbd = keyboard::Keyboard::open().unwrap();
            kbd.keylock().unwrap();
            kbd.calib().unwrap();
            while dialog::Question::new("hold the key you want to calibrate firmly, then press Yes, when you are done, press No").title("Calib").show().unwrap() == dialog::Choice::Yes {
                kbd.calib_press().unwrap();
            }
            kbd.keyunlock().unwrap();
        }
    }
}
