
use lexer;

use super::Error;
use lexer::stream_lexer::{Lexeme,Cursor};

pub trait CursorExt<'a>{
    #[inline(always)]
    fn next_lexeme(&mut self) -> Result<Lexeme<'a>,Error<'a>>;
}

impl<'a> CursorExt<'a> for Cursor<'a>{
    fn next_lexeme(&mut self) -> Result<Lexeme<'a>,Error<'a>>{
        match self.next_lex(){
            Ok( lex ) => Ok( lex ),
            Err( e ) => Err( Error::LexerError(e) ),
        }
    }
}
