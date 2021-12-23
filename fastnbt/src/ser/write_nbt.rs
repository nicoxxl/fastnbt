use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use crate::error::Result;
use crate::Tag;

pub(crate) trait WriteNbt: Write {
    fn write_tag(&mut self, tag: Tag) -> Result<()> {
        self.write_u8(tag as u8)?;
        Ok(())
    }

    fn write_size_prefixed_str(&mut self, key: &str) -> Result<()> {
        let key = cesu8::to_java_cesu8(key);
        let len_bytes = key.len() as u16;
        self.write_u16::<BigEndian>(len_bytes)?;
        self.write_all(&key)?;
        Ok(())
    }
}

impl<T> WriteNbt for T where T: Write {}
