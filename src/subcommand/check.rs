use std::io::IsTerminal;
use globset::{Glob, GlobSetBuilder};
use grep::searcher::{BinaryDetection, SearcherBuilder};
use grep::regex::RegexMatcherBuilder;
use grep::printer::{ColorSpecs, StandardBuilder};
use ignore::Walk;
use termcolor::{BufferWriter, ColorChoice};

use crate::config::Config;


pub fn execute(conf: Config) {
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

        let mut builder = RegexMatcherBuilder::new();
        builder.case_insensitive(!rule.pattern.case_sensitive);
        if rule.pattern.multiline {
            builder.multi_line(rule.pattern.multiline);
        } else {
            builder.line_terminator(Some(b'\n'));
        };
        let matcher = if rule.pattern.string.is_some() {
            builder.build(rule.pattern.string.unwrap().as_str())
        } else {
            builder.build(rule.pattern.regexp.unwrap().as_str())
        };
        if matcher.is_err() {
            eprintln!("regex error: {}", matcher.err().unwrap());
            continue;
        }
        let matcher = matcher.unwrap();

        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .line_number(true)
            .multi_line(rule.pattern.multiline)
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

            let bufwtr = BufferWriter::stdout(if std::io::stdout().is_terminal() {
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
                eprintln!("error: {}: {}", p.display(), err);
            } else {
                let output = String::from_utf8(printer.into_inner().into_inner()).unwrap();
                for line in output.lines() {
                    let o = line.trim_end().trim_start_matches("./");
                    if o.starts_with("goodcheck.yml:") {
                        continue;
                    }
                    println!("{}:\t{}", o, rule.message);
                }
            }
        }
    }
}
