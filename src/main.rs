#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod util;
mod subcommand;
mod config;

use clap::Command;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .subcommand_required(true)
        .version(env!("CARGO_PKG_VERSION"))
        .about("goodcheck clone")
        .subcommand(Command::new("init").about("Generate a sample configuration file"))
        .subcommand(Command::new("check").about("Run check with a configuration"))
        .subcommand(Command::new("test").about("Test configuration file"))
        .subcommand(Command::new("version").about("Print version"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

    if let Some(_matches) = matches.subcommand_matches("init") {
        subcommand::init::execute();
        return;
    }

    let conf = config::load_config(config::DEFAULT_CONF);

    if let Some(_matches) = matches.subcommand_matches("test") {
        subcommand::test::execute(&conf);

        return;
    }

    if let Some(_matches) = matches.subcommand_matches("check") {
        match conf {
            Ok(_) => {}
            Err(_e) => {
                eprintln!("fail to read from config file: {}", _e);
                return;
            }
        }
        let conf = conf.unwrap();
        subcommand::check::execute(conf);
    }
}
