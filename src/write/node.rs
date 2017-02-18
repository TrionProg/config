use std;

use super::Error;
use super::Writer;

use super::Association;
use super::Enum;
use super::List;
use super::Struct;

pub enum Node<'a>{
    Integer(i64),
    Float(f64),
    Text(String),
    TextRef(&'a str),
    Association(Association<'a>),
    Enum(Enum<'a>),
    List(List<'a>),
    Struct(Struct<'a>),
}

impl<'a> Node<'a> {
    ///Decodes Node starting with cur.lex. After call cur.lex is lexeme after Node
    pub fn write<W>(&self, writer:&mut Writer<W> ) -> Result<(), Error> where W:std::io::Write {
        match *self{
            Node::Integer( value ) => writer.write_str( value.to_string().as_ref() ),
            Node::Float( value ) => writer.write_str( value.to_string().as_ref() ),
            Node::Text( ref text ) => writer.write_text( text ),
            Node::TextRef( text ) => writer.write_text( text ),
            Node::Association( ref association ) => association.write(writer),
            Node::Enum( ref enumeration ) => enumeration.write(writer),
            Node::List( ref list ) => list.write(writer),
            Node::Struct( ref structure ) => structure.write(writer, false),
        }
    }
}

impl<'a> From<&'a str> for Node<'a>{
    fn from(text_ref:&'a str) -> Self{
        Node::TextRef( text_ref )
    }
}
