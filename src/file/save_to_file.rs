use std::{error::Error, fs::File, io::Write};

use crate::DotIni;

impl<'a> DotIni<'a> {
    pub fn save_to_file(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut buff = String::default();
        for (section, values) in self.inner {
            buff += &format!("[{section}]\r\n");
            for (key, value) in values {
                buff += &format!("{key}={value}\r\n");
            }
            buff += &format!("\r\n");
        }
        let mut file = File::options().write(true).open(self.path)?;
        file.set_len(0)?;
        file.write_all(buff.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use super::*;

    #[test]
    fn test_save_to_file() {
        let mut ini = DotIni::new(Path::new("test.ini")).load_file().unwrap();

        ini.inner.insert(String::from("test"), HashMap::new());

        assert!(ini.save_to_file().is_ok());
    }
}
