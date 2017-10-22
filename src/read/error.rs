use std;
use lexer;

use super::Location;

///Describes what error has been occured and where
#[derive(Debug)]
pub enum Error<'a>{
    LexerError(lexer::Error),
    RootStructEndsBracket(lexer::Line),
    UnexpectedLexeme(lexer::Line, lexer::stream_lexer::Lexeme<'a>, &'static str),
    StructContainsNoField(Location, Option<&'a str>, &'a str),

    ExpectedInteger(Location, &'static str),
    ExpectedFloat(Location, &'static str),
    ExpectedText(Location, &'static str),
    ExpectedAssociation(Location, &'static str),
    ExpectedEnum(Location, &'static str),
    ExpectedList(Location, &'static str),
    ExpectedStruct(Location, &'static str),

    ReadIOResult(std::io::Error),

    Other(String),
}

//TODO: Other must stora location or line

impl<'a> std::fmt::Display for Error<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::LexerError(ref e) => write!(f, "{}", e),
            Error::RootStructEndsBracket(ref line) => write!(f, "Root struct should not ends with '{}'\n{}", "}", line),
            Error::UnexpectedLexeme(ref line, ref lexeme, expected) => write!(f, "Expected {} , but {} has been found\n{}",expected,lexeme,line),
            Error::StructContainsNoField(ref location, ref struct_name, field_name) => {
                match *struct_name{
                    Some( struct_name ) => write!(f, "{} Struct \"{}\" does not contains field \"{}\"", location, struct_name, field_name),
                    None => write!(f, "{} Anonymous struct does not contains field \"{}\"", location, field_name),
                }
            },
            Error::ExpectedInteger(ref location, found) => write!(f, "{} Expected integer, but {} has been found", location, found),
            Error::ExpectedFloat(ref location, found) => write!(f, "{} Expected float, but {} has been found", location, found),
            Error::ExpectedText(ref location, found) => write!(f, "{} Expected text, but {} has been found", location, found),
            Error::ExpectedAssociation(ref location, found) => write!(f, "{} Expected association, but {} has been found", location, found),
            Error::ExpectedEnum(ref location, found) => write!(f, "{} Expected enumeration, but {} has been found", location, found),
            Error::ExpectedList(ref location, found) => write!(f, "{} Expected list, but {} has been found", location, found),
            Error::ExpectedStruct(ref location, found) => write!(f, "{} Expected struct, but {} has been found", location, found),

            Error::ReadIOResult(ref e) => write!(f, "IO Read Error:{}", e),

            Error::Other(ref message) => write!(f, "{}\n", message),
        }
    }
}

/*
impl<'a> From< Result<lexer::stream_lexer::Lexeme,lexer::Error> > for Error<'a>{
    fn from(lexer_error:Result<lexer::stream_lexer::Lexeme,lexer::Error>,
*/
