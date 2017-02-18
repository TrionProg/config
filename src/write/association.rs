use std;

use super::Error;
use super::Writer;

use super::Node;

pub struct Association<'a>{
    pub key:Box<Node<'a>>,
    pub value:Box<Node<'a>>,
}

impl<'a> Association<'a>{
    pub fn new(key:Node<'a>, value:Node<'a>) -> Self {
        Association{
            key:Box::new(key),
            value:Box::new(value),
        }
    }

    pub fn write<W>(&self, writer:&mut Writer<W> ) -> Result<(), Error> where W:std::io::Write {
        self.key.write(writer)?;
        writer.write_str(" => ")?;
        self.value.write(writer)?;

        Ok(())
    }
}
