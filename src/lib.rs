use std::{collections::HashMap, path::Path};

pub mod file;
pub mod getters_and_setters;

/// Welcome to DotIni a simple dot.ini parser
/// # loading values:
///  ```rust ignore
///     let ini = DotIni::new("path/to/file.ini").load_file().unwrap();
///     ```
///  # getting a value loaded:
///  ```rust ignore
///     let ini = DotIni::new("path/to/file.ini").load_file().unwrap();
///     let section = String::from("mysection");
///     let property = String::from("myproperty");
///     let value = ini.get(section,property);
///     ```
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
