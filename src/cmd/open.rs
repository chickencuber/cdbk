use std::{
    fs,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    thread,
};

use cmdparsing::define;
use image::GenericImageView;
use slint::{Rgba8Pixel, SharedPixelBuffer};
use zip::ZipArchive;

use crate::{
    cmd::{extract, install},
    manifest::Manifest,
};

const HELP: &str = r#"usage: cdbk install [file path]"#;

define! {
    Arguments;
    help: HELP;
    flags {};
    args {
        input: PathBuf,
    };
}

slint::include_modules!();

pub fn open(mut v: Vec<String>) {
    let window = Installer::new().unwrap();

    v.remove(0);
    let args = Arguments::from(v);
    let input = args.input.to_str().unwrap().to_string();

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
    let mut buf = String::new();
    zip.by_name("manifest.toml")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let manifest: Manifest = toml::from_str(&buf).unwrap();
    window.set_app_name(manifest.name.into());
    if let Some(icon) = manifest.icon {
        let mut buf = Vec::new();
        zip.by_path(icon).unwrap().read_to_end(&mut buf).unwrap();
        if let Ok(img) = image::load_from_memory(&buf) {
            let (width, height) = img.dimensions();
            let rgba = img.to_rgba8().to_vec();
            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(&rgba, width, height);
            let slint_image = slint::Image::from_rgba8(buffer);
            window.set_app_icon(slint_image);
        } else {
            window.set_app_icon(slint::Image::default());
        }
    } else {
        window.set_app_icon(slint::Image::default());
    }

    window.on_install_clicked({
        let window_handle = window.as_weak();
        let input = input.clone();
        move || {
            let window = window_handle.unwrap();
            window.set_action(Action::Install);
            window.set_progress(true);
            install(vec!["".into(), input.clone()]);
            window.set_page(Page::Done);
        }
    });
    window.on_extract_clicked({
        let window_handle = window.as_weak();
        move || {
            let window_handle = window_handle.clone();
            let input = input.clone();
            thread::spawn(move || {
                let Some(folder) = rfd::FileDialog::new()
                    .set_can_create_directories(true)
                    .pick_folder()
                else {
                    return;
                };
                slint::invoke_from_event_loop(move || {
                    let Some(window) = window_handle.upgrade() else {
                        return;
                    };
                    window.set_action(Action::Extract);
                    window.set_progress(true);
                    extract(vec![
                        "".into(),
                        input.clone(),
                        "-o".into(),
                        folder.to_str().unwrap().to_string(),
                    ]);

                    window.set_page(Page::Done);
                })
                .unwrap();
            });
        }
    });
    window.run().unwrap()
}
