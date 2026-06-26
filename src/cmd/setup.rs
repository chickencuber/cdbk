use std::{fs, os::unix::fs::PermissionsExt, process::Command};

use cmdparsing::define;

use crate::locations::icon_dir;

const HELP: &str = r#"usage: cdbk setup"#;

define! {
    Arguments;
    help: HELP;
    flags {};
    args {};
}

pub fn setup(mut v: Vec<String>) {
    v.remove(0);
    Arguments::from(v);
    let home = std::env::var("HOME").unwrap();

    //TASK(20260625-191810-745-n6-308): make this use locations stuff
    let mime_dir = format!("{}/.local/share/mime/packages", home);
    let apps_dir = format!("{}/.local/share/applications", home);
    let mime_root = format!("{}/.local/share/mime", home);
    let mut icon_dir = icon_dir().unwrap();
    icon_dir.push("application-x-cdbk.svg");
    fs::write(
        icon_dir,
        include_str!("../resources/application-x-cdbk.svg"),
    )
    .unwrap();

    fs::create_dir_all(&mime_dir).unwrap();
    fs::create_dir_all(&apps_dir).unwrap();

    fs::write(
        format!("{}/x-cdbk.xml", mime_dir),
        include_str!("../resources/x-cdbk.xml"),
    )
    .unwrap();

    fs::write(
        format!("{}/cdbk.desktop", apps_dir),
        include_str!("../resources/cdbk.desktop"),
    )
    .unwrap();

    let mut perms = fs::metadata(format!("{}/cdbk.desktop", apps_dir))
        .unwrap()
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(format!("{}/cdbk.desktop", apps_dir), perms).unwrap();

    // update MIME DB
    Command::new("update-mime-database")
        .arg(&mime_root)
        .status()
        .expect("failed to update mime database");

    let _ = Command::new("update-desktop-database")
        .arg(&apps_dir)
        .status();
    Command::new("xdg-mime")
        .args(["default", "cdbk.desktop", "application/x-cdbk"])
        .status()
        .unwrap();
}
