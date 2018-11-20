#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate clap;
extern crate globset;
extern crate grep;
extern crate ignore;
extern crate serde_yaml;
extern crate termcolor;
extern crate void;

#[macro_use]
pub mod util;
mod subcommand;
mod config;

use clap::{App, AppSettings, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::SubcommandRequired)
        .version(env!("CARGO_PKG_VERSION"))
        .about("goodcheck clone")
        .subcommand(SubCommand::with_name("init").about("Generate a sample configuration file"))
        .subcommand(SubCommand::with_name("check").about("Run check with a configuration"))
        .subcommand(SubCommand::with_name("test").about("Test configuration file"))
        .subcommand(SubCommand::with_name("version").about("Print version"))
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
