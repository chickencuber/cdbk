use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

use cmdparsing::define;
use minijinja::render;

use crate::{
    cmd::extract,
    locations::{applications_dir, data_dir, rename},
    manifest::Manifest,
};

const HELP: &str = r#"usage: cdbk install [file path]"#;

define! {
    Arguments;
    help: HELP;
    flags {};
    args {
        input: String, //even though its techincally a pathbuf, since im reusing another command
                       //this will make it easier
    };
}

fn find_available() -> Option<String> {
    for i in 0..=255 {
        let path = format!("./temp{}", i);
        if let Ok(true) = fs::exists(&path) {
        } else {
            return Some(path);
        }
    }
    return None;
}

pub fn install(mut v: Vec<String>) {
    v.remove(0);
    let args = Arguments::from(v);
    let mut path = "./temp".to_string();
    if let Ok(true) = fs::exists(&path) {
        path = find_available().expect("can't create temp file");
    }
    extract(vec![
        String::new(),
        args.input,
        "-o".to_string(),
        path.clone(),
    ]); //reusing code in a stupid way
    let mut p = PathBuf::from(&path);
    p.push("manifest.toml");
    let Ok(data) = fs::read_to_string(&p) else {
        eprintln!("error parsing manifest.toml");
        return;
    };
    let Ok(manifest) = toml::from_str::<Manifest>(&data) else {
        eprintln!("error parsing manifest.toml");
        return;
    };
    //makes the payload executable
    p.pop();
    p.push(&manifest.payload);
    let mut perms = fs::metadata(&p).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&p, perms).unwrap();

    let mut data_dir = data_dir().unwrap();
    data_dir.push(&manifest.name);
    rename(path, &data_dir).unwrap();
    //yes I know exe is the wrong word here
    let mut exe = data_dir.clone();
    exe.push(&manifest.payload);
    let icon = manifest.icon.map(|s| {
        let mut i = data_dir.clone();
        i.push(s);
        return i;
    });
    let mut app = applications_dir().unwrap();
    app.push(format!("cdbk-{}", &manifest.name));
    app.set_extension("desktop");
    let mut exec_dir = exe.clone();
    exec_dir.pop();
    let desktop = render!(include_str!("../resources/template.desktop"),     
        name => &manifest.name,
        exec => format!("{} {}", exe.to_string_lossy(), manifest.args.iter().map(|v| {
            format!("\"{}\"", v) 
        }).collect::<Vec<String>>().join(" ")),
        exec_dir => exec_dir.to_string_lossy(),
        terminal => manifest.terminal,
        description => manifest.description.as_deref(),
        icon => icon.as_ref().map(|p| p.to_string_lossy()),
        categories => &manifest.categories,
        BIN => &std::env::current_exe().unwrap());
    fs::write(&app, desktop).unwrap();

    let mut perms = fs::metadata(&app).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&app, perms).unwrap();
}
