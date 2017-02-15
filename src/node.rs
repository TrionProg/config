use std;
use lexer;

use lexer::stream_lexer::Lexeme;
use CursorExt;

use Error;
use Location;

use Integer;
use Float;
use Text;
use Association;
use Enum;
use List;
use Struct;

pub enum Node<'a>{
    Integer(Integer),
    Float(Float),
    Text(Text<'a>),
    Association(Association<'a>),
    Enum(Enum<'a>),
    List(List<'a>),
    Struct(Struct<'a>),
}

//IDEA:add Option or enums = Some(...)

impl<'a> Node<'a> {
    ///Decodes Node starting with cur.lex. After call cur.lex is lexeme after Node
    pub fn parse( cur:& mut lexer::stream_lexer::Cursor<'a> ) -> Result<Node<'a>, Error<'a>>{
        //TODO:Add association
        //TODO:Add floats
        let location=Location::new(cur);

        let node=match cur.lex {
            Lexeme::Ident( s ) if s=="None" => {
                Node::Enum( Enum::none(cur, s, location)? )
            },
            Lexeme::String( s ) | Lexeme::Ident( s ) => {
                match cur.next_lexeme()?{
                    Lexeme::Bracket('(') => Node::Enum( Enum::parse(cur, s, location)? ),
                    Lexeme::Bracket('[') => Node::List( List::parse(cur, Some(s), location)? ),
                    Lexeme::Bracket('{') => Node::Struct( Struct::parse(cur, Some(s), location, false)? ),
                    Lexeme::Operator('=') => Node::Association( Association::parse(cur,Node::Text(Text::new(location, s)))? ),
                    _ => Node::Text(Text::new(location, s)),
                }
            },
            Lexeme::Number( value ) => {
                let node=match cur.next_lexeme()?{
                    Lexeme::Dot => Node::Float( Float::parse(cur,false,value as i64, location)? ),
                    _ => Node::Integer(Integer::new(location,value as i64)),
                };

                match cur.lex {
                    Lexeme::Operator('=') => Node::Association( Association::parse(cur,node)? ),
                    _ => node
                }
            },
            Lexeme::Operator('-') => {
                match cur.next_lexeme()?{
                    Lexeme::Number( value ) => {
                        let node=match cur.next_lexeme()?{
                            Lexeme::Dot => Node::Float( Float::parse(cur,true,value as i64, location)? ),
                            _ => Node::Integer(Integer::new(location,-(value as i64))),
                        };

                        match cur.lex {
                            Lexeme::Operator('=') => Node::Association( Association::parse(cur,node)? ),
                            _ => node
                        }
                    },
                    _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"number after -") ),
                }

            },
            Lexeme::Bracket('[') =>
                Node::List( List::parse(cur, None, location)? ),
            Lexeme::Bracket('{') =>
                Node::Struct( Struct::parse(cur, None, location, false)? ),
            _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"string, number, '{' or '['") ),
        };

        Ok(node)
    }

    pub fn get_integer(&self) -> Result<&Integer,Error<'a>>{
        match *self{
            Node::Integer( ref integer ) => Ok(integer),
            _ => Err( Error::ExpectedInteger(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn get_float(&self) -> Result<&Float,Error<'a>>{
        match *self{
            Node::Float( ref float ) => Ok(float),
            _ => Err( Error::ExpectedFloat(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn get_text(&self) -> Result<&Text<'a>,Error<'a>>{
        match *self{
            Node::Text( ref text ) => Ok(text),
            _ => Err( Error::ExpectedText(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn get_assoc(&self) -> Result<&Association<'a>,Error<'a>>{
        match *self{
            Node::Association( ref association ) => Ok(association),
            _ => Err( Error::ExpectedAssociation(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn get_enum(&self) -> Result<&Enum<'a>,Error<'a>>{
        match *self{
            Node::Enum( ref enumeration ) => Ok(enumeration),
            _ => Err( Error::ExpectedEnum(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn get_list(&self) -> Result<&List<'a>,Error<'a>>{
        match *self{
            Node::List( ref list ) => Ok(list),
            _ => Err( Error::ExpectedList(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn get_struct(&self) -> Result<&Struct<'a>,Error<'a>>{
        match *self{
            Node::Struct( ref structure ) => Ok(structure),
            _ => Err( Error::ExpectedStruct(self.get_location().clone(), self.print_type()) ),
        }
    }

    pub fn print_type(&self) -> &'static str{
        match *self{
            Node::Integer(_) => "integer",
            Node::Float(_) => "float",
            Node::Text(_) => "text",
            Node::Association(_) => "association",
            Node::Enum(_) => "enumeration",
            Node::List(_) => "list",
            Node::Struct(_) => "struct",
        }
    }

    pub fn get_location(&self) -> &Location {
        match *self{
            Node::Integer( ref integer ) => &integer.location,
            Node::Float( ref float ) => &float.location,
            Node::Text( ref text ) => &text.location,
            Node::Association( ref association) => &association.location,
            Node::Enum( ref enumeration ) => &enumeration.location,
            Node::List( ref list ) => &list.location,
            Node::Struct( ref structure ) => &structure.location,
        }
    }
}
