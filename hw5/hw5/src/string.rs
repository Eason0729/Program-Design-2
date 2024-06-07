use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStrExt as _,
};

pub trait OsStrExt {
    fn remove_extension(&self) -> &OsStr;
}

impl OsStrExt for OsStr {
    fn remove_extension(&self) -> &OsStr {
        self.as_bytes()
            .iter()
            .rposition(|&x| x == b'.')
            .map(|i| OsStr::from_bytes(&self.as_bytes()[..i]))
            .unwrap_or(self)
    }
}

pub trait OsStringExt {
    fn add_extension(&self, ext: &str) -> OsString;
}

impl OsStringExt for OsString {
    fn add_extension(&self, ext: &str) -> OsString {
        let mut os_string = self.clone();
        os_string.push(".");
        os_string.push(ext);
        os_string
    }
}
