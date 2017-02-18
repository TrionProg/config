use std;

use super::Error;
use super::Writer;

use super::Node;
use super::SplitElementsWith;

pub struct Enum<'a>{
    pub variant_name: &'a str,
    pub elements: Vec<Node<'a>>,
    pub split_elements_with: SplitElementsWith,
}

impl<'a> Enum<'a>{
    pub fn new(variant_name:&'a str, split_elements_with:SplitElementsWith) -> Self{
        Enum{
            variant_name:variant_name,
            elements: Vec::new(),
            split_elements_with:split_elements_with,
        }
    }

    pub fn add_element(&mut self, element:Node<'a>){
        self.elements.push( element );
    }

    pub fn write<W>(&self, writer:&mut Writer<W> ) -> Result<(), Error> where W:std::io::Write {
        if self.variant_name=="None" {
            writer.write_str("None")?;
        }else if self.elements.len()==0 {
            writer.write_text( self.variant_name )?;
            writer.write_str("()\n")?;
        }else{
            writer.write_text( self.variant_name )?;
            writer.write_str("( ")?;

            for element in self.elements.iter().take(self.elements.len()-1) {
                element.write( writer )?;
                writer.write_str( self.split_elements_with.get_str() )?;
            }

            self.elements.last().unwrap().write(writer)?;
            writer.write_str(" )")?;
        }

        Ok(())
    }
}
