use std::{collections::HashMap, sync::Arc};

pub mod file;

#[derive(Default)]
pub struct DotIni {
    inner: HashMap<String, HashMap<String, String>>,
}

impl DotIni {
    pub fn new() -> DotIni {
        DotIni {
            ..Default::default()
        }
    }
}
