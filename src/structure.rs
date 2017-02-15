use std;
use lexer;

use Error;
use lexer::stream_lexer::Lexeme;
use CursorExt;

use Location;

use Integer;
use Float;
use Text;
use Association;
use Enum;
use List;
use Node;

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

pub struct Struct<'a>{
    pub name:Option<&'a str>,
    pub location:Location,
    pub fields:BTreeMap<&'a str, Node<'a>>,
}

impl<'a> Struct<'a> {
    ///Decodes struct. This function calls after { or at beginning of document, then is_root=true. After call cur.lex is EOF or next lexeme after }
    pub fn parse( cur:& mut lexer::stream_lexer::Cursor<'a>, name:Option<&'a str>, location:Location, is_root:bool ) -> Result<Struct<'a>, Error<'a>>{
        let mut fields=BTreeMap::new();

        cur.next_lexeme()?;

        loop{
            let field_name=match cur.lex{
                Lexeme::String( s ) | Lexeme::Ident( s ) => s,
                Lexeme::Bracket('}') => {
                    if is_root {
                        return Err( Error::RootStructEndsBracket(cur.get_line()) );
                    }else{
                        cur.next_lexeme()?;
                        break;
                    }
                },
                Lexeme::EOF => {
                    if is_root {
                        break;
                    }else{
                        return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"'}'") );
                    }
                },
                _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"name of field or '}'") ),
            };

            match cur.next_lexeme()?{
                Lexeme::Operator('=') | Lexeme::Colon => {},
                _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"'=' or ':'") ),
            }

            cur.next_lexeme()?;

            let field_value=Node::parse(cur)?;

            match fields.entry(field_name){
                Entry::Occupied(_) => return Err( Error::Other(format!("Struct already contains fields with name \"{}\"",field_name)) ),
                Entry::Vacant(e) => {
                    e.insert(field_value);
                },
            }

            match cur.lex{
                Lexeme::Comma | Lexeme::Semicolon => {cur.next_lexeme()?;},
                _ => {},
            }
        }

        Ok(
            Struct{
                name:name,
                location:location,
                fields:fields,
            }
        )
    }

    pub fn get_integer(&self,field_name:&'a str) -> Result<&Integer,Error<'a>>{
        self.get_field(field_name)?.get_integer()
    }

    pub fn get_float(&self,field_name:&'a str) -> Result<&Float,Error<'a>>{
        self.get_field(field_name)?.get_float()
    }

    pub fn get_text(&self,field_name:&'a str) -> Result<&Text<'a>,Error<'a>>{
        self.get_field(field_name)?.get_text()
    }

    pub fn get_assoc(&self,field_name:&'a str) -> Result<&Association<'a>,Error<'a>>{
        self.get_field(field_name)?.get_assoc()
    }

    pub fn get_enum(&self,field_name:&'a str) -> Result<&Enum<'a>,Error<'a>>{
        self.get_field(field_name)?.get_enum()
    }

    pub fn get_list(&self,field_name:&'a str) -> Result<&List<'a>,Error<'a>>{
        self.get_field(field_name)?.get_list()
    }

    pub fn get_struct(&self,field_name:&'a str) -> Result<&Struct<'a>,Error<'a>>{
        self.get_field(field_name)?.get_struct()
    }


    fn get_field(&self,field_name:&'a str) -> Result<&Node<'a>,Error<'a>>{
        match self.fields.get(field_name){
            Some( ref field ) => Ok( field ),
            None => Err( Error::StructContainsNoField(self.location.clone(), self.name, field_name) )
        }
    }

    //TODO: add reders
}
