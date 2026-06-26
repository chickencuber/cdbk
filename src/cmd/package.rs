use std::{
    fs::{self, File, remove_file},
    io::Write,
    path::PathBuf,
};

use cmdparsing::define;
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::{locations::rename, manifest::Manifest};

const HELP: &str = r#"usage: cdbk package [file path]
===== FLAGS ===== 
-o|--out [path]: the output path
"#;

define! {
    Arguments;
    help: HELP;
    flags {
        ouput: PathBuf = "o"|"out",
    };
    args {
        input: PathBuf,
    };
}

pub fn package(mut v: Vec<String>) {
    let mut bak = false;
    v.remove(0);
    let args = Arguments::from(v);
    let out = args.ouput.unwrap_or_else(|| {
        let mut out = args.input.clone();
        out.set_extension("cdbk");
        return out;
    });
    let mut new = out.clone();
    new.add_extension("bak");
    if out.exists() {
        rename(&out, &new).unwrap();
        bak = true;
    }

    let Ok(mut file) = File::create(&out) else {
        eprintln!("error creating file");
        remove_file(&out).unwrap();
        if bak {
            rename(&new, &out).unwrap();
        }
        return;
    };
    file.write(b"CDBK").unwrap();
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    let mut zip = ZipWriter::new(file);
    let mut path = args.input;
    path.push("manifest.toml");
    let Ok(data) = fs::read_to_string(&path) else {
        eprintln!("file {:?} does not exist", path);
        remove_file(&out).unwrap();
        if bak {
            rename(&new, &out).unwrap();
        }
        return;
    };
    zip.start_file("manifest.toml", options).unwrap();
    zip.write_all(data.as_bytes()).unwrap();
    let Ok(manifest) = toml::from_str::<Manifest>(&data) else {
        eprintln!("error parsing manifest.toml");
        remove_file(&out).unwrap();
        if bak {
            rename(&new, &out).unwrap();
        }
        return;
    };
    let payload = manifest.payload;
    if payload.components().count() != 1 {
        eprintln!("incorrect manifest");
        remove_file(&out).unwrap();
        if bak {
            rename(&new, &out).unwrap();
        }
        return;
    }
    path.pop();
    path.push(&payload);
    let Ok(data) = fs::read(&path) else {
        eprintln!("file {:?} does not exist", path);
        remove_file(&out).unwrap();
        if bak {
            rename(&new, &out).unwrap();
        }
        return;
    };
    zip.start_file(payload.to_str().unwrap(), options).unwrap();
    zip.write_all(data.as_slice()).unwrap();

    if let Some(icon) = manifest.icon {
        if icon.components().count() != 1 {
            eprintln!("incorrect manifest");
            remove_file(&out).unwrap();
            if bak {
                rename(&new, &out).unwrap();
            }
            return;
        }
        path.pop();
        path.push(&icon);
        let Ok(data) = fs::read(&path) else {
            eprintln!("file {:?} does not exist", path);
            remove_file(&out).unwrap();
            if bak {
                rename(&new, &out).unwrap();
            }
            return;
        };

        zip.start_file(icon.to_str().unwrap(), options).unwrap();
        zip.write_all(data.as_slice()).unwrap();
    }

    zip.finish().unwrap();

    if bak {
        remove_file(&new).unwrap();
    }
}
