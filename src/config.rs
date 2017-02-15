use std;
use lexer;

use Error;
use Location;

use Integer;
use Float;
use Text;
use Association;
use Enum;
use List;
use Struct;

pub struct Config<'a>{
    root:Struct<'a>,
}

impl<'a> Config<'a>{
    pub fn parse(text:&'a str) -> Result<Config<'a>, Error<'a>>{
        let mut cursor=lexer::stream_lexer::Cursor::new( text );

        let root=Struct::parse(&mut cursor, Some("root"), Location::document_beginning(), true)?;

        Ok(
            Config{
                root:root,
            }
        )
    }

    pub fn get_integer(&self,field_name:&'a str) -> Result<&Integer,Error<'a>>{
        self.root.get_integer(field_name)
    }

    pub fn get_float(&self,field_name:&'a str) -> Result<&Float,Error<'a>>{
        self.root.get_float(field_name)
    }

    pub fn get_text(&self,field_name:&'a str) -> Result<&Text<'a>,Error<'a>>{
        self.root.get_text(field_name)
    }

    pub fn get_assoc(&self,field_name:&'a str) -> Result<&Association<'a>,Error<'a>>{
        self.root.get_assoc(field_name)
    }

    pub fn get_enum(&self,field_name:&'a str) -> Result<&Enum<'a>,Error<'a>>{
        self.root.get_enum(field_name)
    }

    pub fn get_list(&self,field_name:&'a str) -> Result<&List<'a>,Error<'a>>{
        self.root.get_list(field_name)
    }

    pub fn get_struct(&self,field_name:&'a str) -> Result<&Struct<'a>,Error<'a>>{
        self.root.get_struct(field_name)
    }


}
