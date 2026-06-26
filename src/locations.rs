use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn rename<P, Q>(from: P, to: Q) -> fs_extra::error::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if from.as_ref().is_dir() {
        let options = fs_extra::dir::CopyOptions::new().copy_inside(true).overwrite(true);
        fs_extra::dir::move_dir(from, to, &options)?;
    } else {
        let options = fs_extra::file::CopyOptions::new().overwrite(true);
        fs_extra::file::move_file(from, to, &options)?;
    }
    Ok(())
}

pub fn applications_dir() -> Option<PathBuf> {
    if let Some(mut d) = dirs::data_local_dir() {
        d.push("applications");
        if fs::create_dir_all(&d).is_err() {
            return None;
        }
        return Some(d);
    } else {
        return None;
    }
}

pub fn data_dir() -> Option<PathBuf> {
    if let Some(mut d) = dirs::data_local_dir() {
        d.push("cdbk");
        if fs::create_dir_all(&d).is_err() {
            return None;
        }
        return Some(d);
    } else {
        return None;
    }
}

pub fn icon_dir() -> Option<PathBuf> {
    if let Some(mut d) = dirs::data_local_dir() {
        d.push("icons");
        d.push("hicolor");
        d.push("16x16");
        d.push("mimetypes");
        if fs::create_dir_all(&d).is_err() {
            return None;
        }
        return Some(d);
    } else {
        return None;
    }
}
