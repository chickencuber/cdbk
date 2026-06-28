use std::fs;

use cmdparsing::define;

use crate::locations::{applications_dir, data_dir};

const HELP: &str = r#"usage: cdbk remove [file path]"#;

define! {
    Arguments;
    help: HELP;
    flags {};
    args {
        bundle: String,
    };
    rest=>rest:String;
}

pub fn remove(mut v: Vec<String>) {
    v.remove(0);
    let args = Arguments::from(v);
    let mut data = data_dir().unwrap();
    data.push(&args.bundle);
    let mut app = applications_dir().unwrap();
    app.push(format!("cdbk-{}", &args.bundle));
    app.set_extension("desktop");
    if app.exists() {
    fs::remove_file(app).unwrap();
    fs::remove_dir_all(data).unwrap();
    } else {
        eprintln!("bundle '{}' doesn't exist", args.bundle);
    }
    if args.rest.len() > 0 {
        remove(vec![String::new()].iter().map(|v| v.clone()).chain(args.rest).collect())
    }
}
