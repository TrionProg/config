use std;
use lexer;

use Error;
use lexer::stream_lexer::Lexeme;
use CursorExt;

use Location;
use Node;

pub struct Enum<'a>{
    pub variant_name: &'a str,
    pub location:Location,
    pub elements: Vec<Node<'a>>,
}

impl<'a> Enum<'a>{
    ///Calls on None ident
    pub fn none( cur:& mut lexer::stream_lexer::Cursor<'a>, variant_name:&'a str, location:Location ) -> Result<Enum<'a>, Error<'a>>{
        cur.next_lexeme()?;

        Ok(
            Enum{
                variant_name:variant_name,
                location:location,
                elements:Vec::new(),
            }
        )
    }

    ///Decodes Enum. This function calls after (. After call cur.lex is lexeme after )
    pub fn parse( cur:& mut lexer::stream_lexer::Cursor<'a>, variant_name:&'a str, location:Location ) -> Result<Enum<'a>, Error<'a>>{
        let mut elements=Vec::new();

        cur.next_lexeme()?;

        loop{
            match cur.lex {
                Lexeme::Bracket(')') => break, //() or (x,)
                Lexeme::Comma => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"element of enumeration") ),
                _ => {},
            }

            elements.push( Node::parse(cur)? );

            match cur.lex {
                Lexeme::Bracket(')') => break,
                Lexeme::Comma => {cur.next_lexeme()?;},
                _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"',' or ')'") ),
            }
        }

        cur.next_lexeme()?;

        Ok(
            Enum{
                variant_name:variant_name,
                location:location,
                elements:elements,
            }
        )
    }

    pub fn is_none(&self ) -> bool {
        self.variant_name=="None"
    }

    pub fn is_some(&self ) -> bool {
        self.variant_name=="Some"
    }

    pub fn iter(&self) -> std::slice::Iter<Node<'a>>{
        self.elements.iter()
    }
}
