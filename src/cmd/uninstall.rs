use std::{fs, process::Command};

use cmdparsing::define;

use crate::{cmd::remove, locations::{data_dir, icon_dir}};

const HELP: &str = r#"usage: cdbk uninstall"#;

define! {
    Arguments;
    help: HELP;
    flags {};
    args {};
}

pub fn uninstall(mut v: Vec<String>) {
    v.remove(0);
    Arguments::from(v);


    for d in fs::read_dir(data_dir().unwrap()).unwrap().into_iter() {
       remove(vec![String::new(), d.unwrap().file_name().to_str().unwrap().to_string()]); 
    }

    let home = std::env::var("HOME").unwrap();

    //TASK(20260625-191810-745-n6-308): make this use locations stuff
    let mime_dir = format!("{}/.local/share/mime/packages", home);
    let apps_dir = format!("{}/.local/share/applications", home);
    let mime_root = format!("{}/.local/share/mime", home);
    let mut icon_dir = icon_dir().unwrap();
    icon_dir.push("application-x-cdbk.svg");
    fs::remove_file(
        icon_dir,
    ).unwrap();

    fs::remove_file(
        format!("{}/x-cdbk.xml", mime_dir),
    ).unwrap();

    fs::remove_file(
        format!("{}/cdbk.desktop", apps_dir),
    ).unwrap();
    fs::remove_dir_all(data_dir().unwrap()).unwrap();

    // update MIME DB
    Command::new("update-mime-database")
        .arg(&mime_root)
        .status()
        .expect("failed to update mime database");

    let _ = Command::new("update-desktop-database")
        .arg(&apps_dir)
        .status();
}
