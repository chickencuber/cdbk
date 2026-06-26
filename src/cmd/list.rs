use std::fs;

use cmdparsing::define;

use crate::locations::data_dir;

const HELP: &str = r#"usage: cdbk list"#;

define! {
    Arguments;
    help: HELP;
    flags {};
    args {};
}

pub fn list(mut v: Vec<String>) {
    v.remove(0);
    Arguments::from(v);
    for d in fs::read_dir(data_dir().unwrap()).unwrap().into_iter() {
        println!("{}", d.unwrap().file_name().to_string_lossy());
    }
}
