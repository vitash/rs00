use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
// use std::os::unix;
use std::path::Path;

// `% cat path` 的简单实现
fn cat(path: &Path) -> io::Result<String> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
