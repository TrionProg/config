use std;

///Describes what error has been occured and where
#[derive(Debug)]
pub enum Error{
    WriteIOResult(std::io::Error),

    Other(String),
}

//TODO: Other must stora location or line

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::WriteIOResult(ref e) => write!(f, "IO Write Error:{}", e),

            Error::Other(ref message) => write!(f, "{}\n", message),
        }
    }
}

/*
impl<'a> From< Result<lexer::stream_lexer::Lexeme,lexer::Error> > for Error<'a>{
    fn from(lexer_error:Result<lexer::stream_lexer::Lexeme,lexer::Error>,
*/
