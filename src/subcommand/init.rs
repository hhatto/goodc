use std::fs;
use std::path::Path;
use std::io::{BufWriter, Write};

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

pub fn execute() {
    let yaml_file = Path::new("goodcheck.yml");
    if yaml_file.exists() {
        println!("already exists 'goodcheck.yml' file in the currenct directory📄");
    } else {
        let mut f = BufWriter::new(fs::File::create("goodcheck.yml").unwrap());
        f.write(INIT_TEMPLATE.as_bytes()).unwrap();

        println!("write 'goodcheck.yml' file in the currenct directory👍");
    }
}
