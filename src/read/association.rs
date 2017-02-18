use std;
use lexer;

use super::Error;
use lexer::stream_lexer::Lexeme;
use super::CursorExt;

use super::Location;
use super::Node;

pub struct Association<'a>{
    pub location:Location,
    pub key:Box<Node<'a>>,
    pub value:Box<Node<'a>>,
}

impl<'a> Association<'a>{
    pub fn parse( cur:& mut lexer::stream_lexer::Cursor<'a>, key:Node<'a> ) -> Result<Association<'a>, Error<'a>>{
        let location=key.get_location().clone();

        match cur.next_lexeme()?{
            Lexeme::Operator('>') => {},
            _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"'>' of =>") ),
        }

        cur.next_lexeme()?;

        let value=Node::parse( cur )?;

        Ok(
            Association{
                location:location,
                key:Box::new(key),
                value:Box::new(value),
            }
        )
    }
}
