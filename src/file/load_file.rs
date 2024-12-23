use std::{collections::HashMap, error::Error, fs, io::Read};

use crate::DotIni;

impl<'a> DotIni<'a> {
    pub fn load_file(mut self) -> Result<DotIni<'a>, Box<dyn Error + Send + Sync>> {
        let mut buf: String = String::default();
        fs::File::open(self.path)?.read_to_string(&mut buf)?;
        let mut act_section = String::default();
        for i in buf.split("\n") {
            if i.starts_with("[") {
                act_section = i
                    .trim()
                    .trim_start_matches("[")
                    .trim_end_matches("]")
                    .to_string();
                self.inner.insert(act_section.clone(), HashMap::new());
                continue;
            }
            let mut splitline = i.trim().split("=");
            if splitline.clone().count() > 0 {
                let key = match splitline.next() {
                    None => {
                        continue;
                    }
                    Some(a) => {
                        if a.is_empty() {
                            continue;
                        } else {
                            a
                        }
                    }
                };
                let value = splitline.next().unwrap_or("");
                match self.inner.get_mut(&act_section) {
                    Some(a) => {
                        a.insert(key.to_string(), value.to_string());
                    }
                    None => {}
                }
            }
        }
        println!("{:?}", self.inner);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_load_file() {
        let dotini = DotIni::new(Path::new("test.ini"));

        assert!(dotini.load_file().is_ok())
    }
}
