#![feature(duration_constants)]
use clap::Clap;
use dialoguer::Confirm;
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
struct Pull {
    #[clap(short, long, default_value = "niz/atom66")]
    name: String,
}

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
        SubCommand::Pull(p) => println!(
            "{}",
            config::Keymap::new(
                p.name,
                keyboard::Keyboard::open().unwrap().read_keymap().unwrap(),
            )
            .encode()
            .unwrap()
        ),
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
            if Confirm::new()
                .with_prompt("do you really want to lock your keyboard, you will need another keyboard to unlock")
                .interact().unwrap()
            {
                keyboard::Keyboard::open().unwrap().keylock().unwrap();
            }
        }
        SubCommand::Unlock(_) => {
            keyboard::Keyboard::open().unwrap().keyunlock().unwrap();
        }
        SubCommand::Calib(_) => {
            if !Confirm::new()
                .with_prompt("you will need another keyboard to continue with the calibration process, as you keyboard will be locked, before continuing, make sure that all the keys are released")
               .interact().unwrap()
            {
                return;
            }
            let kbd = keyboard::Keyboard::open().unwrap();
            kbd.keylock().unwrap();
            kbd.calib().unwrap();
            while Confirm::new().with_prompt("hold the key you want to calibrate firmly, then press y from another keyboard, press n when you are done").interact().unwrap() {
                kbd.calib_press().unwrap();
            }
            kbd.keyunlock().unwrap();
        }
    }
}
