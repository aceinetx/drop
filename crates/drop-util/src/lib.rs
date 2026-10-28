mod maybe;
pub use maybe::*;
pub mod table;

use std::path::{Path, PathBuf};

pub fn expand_path_traversal(path: &Path) -> PathBuf {
    let mut new_path = PathBuf::new();
    for i in path.iter() {
        if i == ".." {
            new_path.pop();
        } else if i == "." {
        } else {
            new_path.push(i);
        }
    }
    new_path
}

#[macro_export]
macro_rules! strerr {
    ($x: expr) => {
        ($x).map_err(|e| e.to_string())
    };
}
