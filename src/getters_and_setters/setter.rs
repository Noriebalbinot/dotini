use std::{collections::HashMap, error::Error};

use crate::DotIni;

impl<'a> DotIni<'a> {
    pub fn set_property(mut self, section: String, property: String, value: String) -> DotIni<'a> {
        match self.inner.get_mut(&section) {
            Some(a) => {
                a.insert(property, value);
            }
            None => {
                let mut prop = HashMap::new();
                prop.insert(property, value);
                self.inner.insert(section, prop);
            }
        };
        self
    }
}
