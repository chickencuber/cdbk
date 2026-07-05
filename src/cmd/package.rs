use std::{
    fs::{self, File, remove_file},
    io::Write,
    path::{Component, Path, PathBuf},
};

use cmdparsing::define;
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipWriter, write::{FileOptions, SimpleFileOptions}};

use crate::{locations::rename, manifest::Manifest};

fn first_real_component(p: &Path) -> Option<&std::ffi::OsStr> {
    p.components()
        .find(|c| matches!(c, Component::Normal(_)))
        .map(|c| match c {
            Component::Normal(s) => s,
            _ => unreachable!(),
        })
}

fn zip_dir<S: AsRef<Path>, T: ToString>(zip: &mut ZipWriter<File>, name: T, src: S, options: FileOptions<'_, ()>) {
    zip.add_directory(name.to_string(), options).unwrap();
    let walkdir = WalkDir::new(&src);
    for entry_result in walkdir.into_iter() {
        let entry = entry_result.unwrap();
        let path = entry.path();
        let path_stripped = path.strip_prefix(&src).unwrap();
        let path_as_string = path_stripped
            .to_str()
            .map(str::to_owned)
            .unwrap();
        if path.is_file() {
            
            zip.start_file(format!("{}/{}", name.to_string(), path_as_string), options).unwrap();
            let mut f = File::open(path).unwrap();

            std::io::copy(&mut f, zip).unwrap();
        } else if !path_stripped.as_os_str().is_empty() {
            zip.add_directory(format!("{}/{}", name.to_string(), path_as_string), options).unwrap();
        }
    }
}

const HELP: &str = r#"usage: cdbk package [file path]
===== FLAGS ===== 
-o|--out [path]: the output path
-c|--compress: an option to compress the bundle
"#;

define! {
    Arguments;
    help: HELP;
    flags {
        ouput: PathBuf = "o"|"out",
        compress: bool = "c" | "compress",
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
    let options;
    if args.compress {
        options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    } else {
        options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    }
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
    if payload.components().count() <= 0 {
        eprintln!("incorrect manifest");
        remove_file(&out).unwrap();
        if bak {
            rename(&new, &out).unwrap();
        }
        return;
    } else if payload.components().count() > 1 {
        path.pop();
        let name = first_real_component(&payload).expect("incorrect manifest").to_str().unwrap();
        path.push(name);
        zip_dir(&mut zip, name, &path, options);
    } else {
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
    }

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
