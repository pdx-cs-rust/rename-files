//! Mass-rename files by regex.
//!
//! Bart Massey 2021

use std::ffi::OsString;
use std::path::{Path, PathBuf};

extern crate argwerk;
use regex::bytes as regex;

argwerk::define! {
    /// Rename files according to a regex.
    #[usage = "rename-files <matching> <replace> <files..>"]
    struct ArgList {
        #[required = "matching must be specified"]
        match_pat: String,
        #[required = "replace must be specified"]
        replace_pat: String,
        #[required = "internal error: zero files not allowed"]
        files_list: Vec<OsString>,
    }
    /// Regex to match, pattern for replacement, list of files to process.
    [matching, replace, #[rest(os)] files] => {
        match_pat = Some(matching);
        replace_pat = Some(replace);
        files_list = Some(files);
    }
}

// https://users.rust-lang.org/t/is-there-a-way-to-convert-vec-u8-to-osstring/49970/3

#[cfg(unix)]
mod into_bytes {
    use super::*;

    use std::os::unix::ffi::OsStringExt;

    pub fn into_bytes(source: OsString) -> Vec<u8> {
        source.into_vec()
    }

    #[allow(unused)]
    pub fn into_os_string(source: Vec<u8>) -> OsString {
        OsStringExt::from_vec(source)
    }
}

use into_bytes::*;

// https://users.rust-lang.org/t/fs-rename-overwriting-existing-file/17314/6
#[cfg(unix)]
mod rename_noreplace {
    use super::*;

    use std::ffi::CString;
    use std::os::unix::ffi::OsStringExt;

    fn to_c_string<P: AsRef<Path>>(path: P) -> CString {
        CString::new(path.as_ref().as_os_str().to_owned().into_vec()).unwrap()
    }

    #[cfg(target_os = "linux")]
    pub fn rename_noreplace<P1, P2>(from_name: P1, to_name: P2) -> std::io::Result<()>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let from_name = to_c_string(from_name);
        let to_name = to_c_string(to_name);
        let result = unsafe {
            libc::renameat2(
                libc::AT_FDCWD,
                from_name.as_ptr(),
                libc::AT_FDCWD,
                to_name.as_ptr(),
                libc::RENAME_NOREPLACE,
            )
        };
        match result {
            0 => Ok(()),
            -1 => Err(std::io::Error::last_os_error()),
            r => panic!("internal error: unexpected renameat2 result {}", r),
        }
    }
}
use rename_noreplace::*;

fn run() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = ArgList::args()?;
    let match_re = regex::Regex::new(&args.match_pat)?;
    let subst: Vec<u8> = args.replace_pat.bytes().collect();

    for file in args.files_list {
        let target = PathBuf::from(file);
        let target_bytes = into_bytes(target.as_os_str().to_owned());
        let replacement = PathBuf::from(into_os_string(
            match_re.replace(&target_bytes, &subst).to_vec(),
        ));
        if let Err(err) = rename_noreplace(&target, &replacement) {
            eprintln!(
                "{} ({} → {}): {}",
                target.as_os_str().to_string_lossy(),
                args.match_pat,
                args.replace_pat,
                err,
            );
            std::process::exit(1);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        eprint!("{}", ArgList::help());
        std::process::exit(1);
    }
}
