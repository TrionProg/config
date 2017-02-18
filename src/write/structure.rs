use std;

use super::Error;
use super::Writer;

use super::Association;
use super::Enum;
use super::List;
use super::Node;
use super::SplitFieldsWith;
use super::AssignmentSymbal;

pub struct Struct<'a>{
    pub name:Option<&'a str>,
    pub fields:Vec<(&'a str, Node<'a>)>,
    pub assignment_symbal: AssignmentSymbal,
    pub split_fields_with: SplitFieldsWith,
}

impl<'a> Struct<'a> {
    pub fn new(name:Option<&'a str>, assignment_symbal: AssignmentSymbal, split_fields_with: SplitFieldsWith) -> Self{
        Struct{
            name:name,
            fields:Vec::new(),
            assignment_symbal:assignment_symbal,
            split_fields_with:split_fields_with,
        }
    }

    pub fn root() -> Self{
        Struct::new(Some("root"), AssignmentSymbal::Colon, SplitFieldsWith::Space)
    }

    pub fn add_field(&mut self,field_name:&'a str, field_value:Node<'a>){
        self.fields.push( (field_name, field_value) );
    }


    pub fn write<W>(&self, writer:&mut Writer<W>, is_root:bool ) -> Result<(), Error> where W:std::io::Write {
        if is_root {
            for &(field_name, ref field_value) in self.fields.iter() {
                writer.write_text(field_name)?;

                writer.write_str( self.assignment_symbal.get_str() )?;

                field_value.write( writer )?;
                writer.write_str( self.split_fields_with.get_str() )?;
                writer.write_str("\n")?;
            }
        }else{
            match self.name {
                Some( ref name ) => {
                    writer.write_text( name )?;
                    writer.write_str(" ")?;
                },
                None => {},
            };

            if self.fields.len()==0 {
                writer.write_str("{}")?;
            }else{
                writer.write_str("{\n")?;
                writer.inc_tab();

                for &(field_name, ref field_value) in self.fields.iter() {
                    writer.write_tab()?;
                    writer.write_text(field_name)?;

                    writer.write_str( self.assignment_symbal.get_str() )?;

                    field_value.write( writer )?;
                    writer.write_str( self.split_fields_with.get_str() )?;
                    writer.write_str("\n")?;
                }

                writer.dec_tab();
                writer.write_tab()?;
                writer.write_str("}")?;
            }
        }

        Ok(())
    }
}
