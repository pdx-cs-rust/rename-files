//! Mass-rename files by regex.
//!
//! Bart Massey 2021

use std::ffi::OsString;

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
#[cfg(target_os = "linux")]
mod rename_noreplace {
    use std::ffi::CString;

    pub fn rename_noreplace(from_name: &[u8], to_name: &[u8]) -> std::io::Result<()> {
        let from_name = CString::new(from_name).unwrap();
        let to_name = CString::new(to_name).unwrap();
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
        let target = into_bytes(file);
        let replacement = match_re.replace(&target, &subst).to_vec();
        if let Err(err) = rename_noreplace(&target, &replacement) {
            eprintln!(
                "{} ({} → {}): {}",
                into_os_string(target).to_string_lossy(),
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
