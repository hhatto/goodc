use std::io::{IsTerminal, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::config::Config;


pub fn execute(conf: &std::result::Result<Config, std::boxed::Box<dyn std::error::Error>>) {
    let mut stdout = StandardStream::stdout(if std::io::stdout().is_terminal() {
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
            let msg = e.to_string();
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
}
