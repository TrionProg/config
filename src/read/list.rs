use std;
use lexer;

use super::Error;
use lexer::stream_lexer::Lexeme;
use super::CursorExt;

use super::Location;
use super::Node;

pub struct List<'a>{
    pub name: Option<&'a str>,
    pub location:Location,
    pub elements: Vec<Node<'a>>,
}

impl<'a> List<'a>{
    ///Decodes List. This function calls after [. After call cur.lex is lexeme after ]
    pub fn parse( cur:& mut lexer::stream_lexer::Cursor<'a>, name:Option<&'a str>, location:Location ) -> Result<List<'a>, Error<'a>>{
        let mut elements=Vec::new();

        cur.next_lexeme()?;

        loop{
            match cur.lex {
                Lexeme::Bracket(']') => break, //[] or [x,]
                Lexeme::Comma => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"element of list") ),
                _ => {},
            }

            elements.push( Node::parse(cur)? );

            match cur.lex {
                Lexeme::Bracket(']') => break,
                Lexeme::Comma => {cur.next_lexeme()?;},
                _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"',' or ']'") ),
            }
        }

        cur.next_lexeme()?;

        Ok(
            List{
                name:name,
                location:location,
                elements:elements,
            }
        )
    }

    pub fn iter(&self) -> std::slice::Iter<Node<'a>>{
        self.elements.iter()
    }
}
