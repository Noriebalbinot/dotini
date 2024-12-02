use crate::DotIni;

impl<'a> DotIni<'a> {
    pub fn get(self, section: String, property: String) -> Option<String> {
        match self.inner.get(&section) {
            Some(a) => match a.get(&property) {
                Some(b) => Some(b.to_string()),
                None => None,
            },
            None => None,
        }
    }
}
