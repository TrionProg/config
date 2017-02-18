use std;

use super::Error;
use super::Writer;

use super::Association;
use super::Enum;
use super::List;
use super::Struct;

pub struct Config<'a>{
    root:Struct<'a>,
}

impl<'a> Config<'a>{
    pub fn new() -> Self{
        Config{
            root:Struct::root(),
        }
    }

    pub fn get_root(&mut self) -> &mut Struct<'a>{
        &mut self.root
    }

    pub fn write<W>(&self, writer:&mut Writer<W>) -> Result<(), Error> where W:std::io::Write {
        self.root.write(writer, true)?;

        Ok(())
    }
}
