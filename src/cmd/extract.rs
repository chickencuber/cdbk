use std::{
    fs,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use cmdparsing::define;
use zip::ZipArchive;

const HELP: &str = r#"usage: cdbk extract [file path]
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

pub fn extract(mut v: Vec<String>) {
    v.remove(0);
    let args = Arguments::from(v);
    let out = args.ouput.unwrap_or_else(|| {
        let mut out = args.input.clone();
        out.set_extension("");
        return out;
    });

    let Ok(mut file) = fs::File::open(&args.input) else {
        eprintln!("File {:?} does not exist", args.input);
        return;
    };

    let mut magic = [0u8; 4];

    file.read_exact(&mut magic).unwrap();

    if &magic != b"CDBK" {
        eprintln!("invalid CDBK magic bytes");
        return;
    }
    //skip header
    file.seek(SeekFrom::Start(4)).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    fs::create_dir_all(&out).unwrap();
    zip.extract(out).unwrap();
}
