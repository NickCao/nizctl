#![feature(duration_constants)]
use clap::Clap;
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
}

#[derive(Clap, Debug)]
struct Pull {
    #[clap(short, long, default_value = "niz/atom66")]
    name: String,
}

#[derive(Clap, Debug)]
struct Push {}

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
    }
}
