use std;

use super::Error;
use super::Writer;

use super::Node;
use super::SplitElementsWith;

pub struct List<'a>{
    pub name: Option<&'a str>,
    pub elements: Vec<Node<'a>>,
    pub split_elements_with: SplitElementsWith,
}

impl<'a> List<'a>{
    pub fn new(name:Option<&'a str>, split_elements_with:SplitElementsWith) -> Self{
        List{
            name:name,
            elements: Vec::new(),
            split_elements_with:split_elements_with,
        }
    }

    pub fn add_element(&mut self, element:Node<'a>){
        self.elements.push( element );
    }

    pub fn write<W>(&self, writer:&mut Writer<W> ) -> Result<(), Error> where W:std::io::Write {
        match self.name {
            Some( ref name ) => {
                writer.write_text( name )?;
                writer.write_str(" ")?;
            },
            None => {},
        };

        if self.elements.len()==0 {
            writer.write_str("[]")?;
        }else{
            writer.write_str("[\n")?;
            writer.inc_tab();

            for element in self.elements.iter().take(self.elements.len()-1) {
                writer.write_tab()?;
                element.write( writer )?;
                writer.write_str( self.split_elements_with.get_str() )?;
                writer.write_str("\n")?;
            }

            writer.write_tab()?;
            self.elements.last().unwrap().write(writer)?;
            writer.write_str("\n")?;

            writer.dec_tab();
            writer.write_tab()?;
            writer.write_str("]")?;
        }

        Ok(())
    }
}
