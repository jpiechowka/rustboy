use anyhow::Result;
use std::fs;

pub struct Rom {
    // TODO: private, provide methods to operate on raw content?
    raw_content: Vec<u8>,
}

pub enum RomError {
    TitleParsingError,
}

impl Rom {
    pub fn load_from_file(filepath: &str) -> Result<Rom> {
        let rom = Rom {
            raw_content: fs::read(filepath)?,
        };

        Ok(rom)
    }

    pub fn title(&self) -> Result<String> {
        let title_bytes: Vec<u8> = self.raw_content[0x134..=0x143]
            .iter()
            .take_while(|&&v| v != 0)
            .cloned()
            .collect();

        let title = String::from_utf8(title_bytes)?;
        Ok(title)
    }
}
