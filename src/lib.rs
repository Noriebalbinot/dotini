use std::{collections::HashMap, path::Path};

pub mod file;

#[derive(Clone)]
pub struct DotIni<'a> {
    path: &'a Path,
    inner: HashMap<String, HashMap<String, String>>,
}

impl<'a> DotIni<'a> {
    pub fn new(file: &'a Path) -> DotIni<'a> {
        DotIni {
            inner: HashMap::new(),
            path: &file,
        }
    }
}
