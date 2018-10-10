#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate globset;
extern crate grep;
extern crate ignore;
extern crate serde_yaml;
extern crate termcolor;

use clap::{App, AppSettings, SubCommand};
use globset::{Glob, GlobSetBuilder};
use grep::cli;
use grep::printer::{ColorSpecs, StandardBuilder};
use grep::regex::RegexMatcher;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use ignore::Walk;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod config;
#[macro_use]
mod util;

const INIT_TEMPLATE: &'static str = "rules:
  - id: com.example.1
    pattern: Github
    message: Do you want to write GitHub?
    glob: 
      - \"**/*.rb\"
      - \"**/*.yaml\"
      - \"**/*.yml\"
      - \"**/*.html\"
    fail:
      - Signup via Github
    pass:
      - Signup via GitHub
";

fn main() {
    let matches = App::new("goodc")
        .setting(AppSettings::SubcommandRequired)
        .version("0.1.0")
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
        let yaml_file = Path::new("goodcheck.yml");
        if yaml_file.exists() {
            println!("already exists 'goodcheck.yml' file in the currenct directory📄");
        } else {
            let mut f = BufWriter::new(fs::File::create("goodcheck.yml").unwrap());
            f.write(INIT_TEMPLATE.as_bytes()).unwrap();

            println!("write 'goodcheck.yml' file in the currenct directory👍");
        }
        return;
    }

    let conf = config::load_config(config::DEFAULT_CONF);

    if let Some(_matches) = matches.subcommand_matches("test") {
        let mut stdout = StandardStream::stdout(if cli::is_tty_stdout() {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        });
        let mut fail_config_count = 0;

        // check yaml keys
        match conf {
            Ok(_) => {
                set_ok_color!(stdout);
                write!(&mut stdout, "ok").unwrap();
                set_normal_color!(stdout);
                println!(" - yaml format and config keys");
            }
            Err(e) => {
                fail_config_count += 1;
                let msg = e.description();
                set_fail_color!(stdout);
                write!(&mut stdout, "fail").unwrap();
                set_normal_color!(stdout);
                println!(" - {}", msg);
            }
        }

        // result of checking
        if fail_config_count == 0 {
            set_ok_color!(stdout);
            writeln!(&mut stdout, "configuration is valid").unwrap();
            set_normal_color!(stdout);
        } else {
            set_fail_color!(stdout);
            println!("configuration is invalid. fail: {}", fail_config_count);
            set_normal_color!(stdout);
        }

        return;
    }

    if let Some(_matches) = matches.subcommand_matches("check") {
        match conf {
            Ok(_) => {}
            Err(_e) => {
                println!("fail to read from config file");
                return;
            }
        }
        let conf = conf.unwrap();
        for rule in conf.rules {
            let mut glob_builder = GlobSetBuilder::new();
            match rule.glob {
                Some(g) => {
                    for glob_str in g {
                        glob_builder.add(Glob::new(&glob_str).unwrap());
                    }
                }
                None => {
                    glob_builder.add(Glob::new("./").unwrap());
                }
            }
            let globs = glob_builder.build().expect("fail build globset");

            let matcher = if rule.pattern.string.is_some() {
                RegexMatcher::new_line_matcher(&rule.pattern.string.unwrap()).unwrap()
            } else {
                RegexMatcher::new_line_matcher(&rule.pattern.regexp.unwrap()).unwrap()
            };
            let mut searcher = SearcherBuilder::new()
                .binary_detection(BinaryDetection::quit(b'\x00'))
                .line_number(true)
                .build();

            for result in Walk::new("./") {
                if result.is_err() {
                    continue;
                }
                let entry = result.unwrap();
                let p = entry.path();
                if !p.is_file() || globs.matches(p).is_empty() {
                    continue;
                }

                let bufwtr = BufferWriter::stdout(if cli::is_tty_stdout() {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                });
                let buffer = bufwtr.buffer();
                let mut printer = StandardBuilder::new()
                    .color_specs(ColorSpecs::default_with_color())
                    .build(buffer);
                let result = searcher.search_path(&matcher, p, printer.sink_with_path(&matcher, p));
                if let Err(err) = result {
                    println!("error: {}: {}", p.display(), err);
                } else {
                    let output = String::from_utf8(printer.into_inner().into_inner()).unwrap();
                    for line in output.lines() {
                        let o = line.trim_right().trim_left_matches("./");
                        if o.starts_with("goodcheck.yml:") {
                            continue;
                        }
                        println!("{}:\t{}", o, rule.message);
                    }
                }
            }
        }
    }
}
