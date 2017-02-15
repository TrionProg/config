use std;
use lexer;

use Node;

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

pub struct Map<'a>{
    pub name:Option<&'a str>,
    pub location:Location,
    pub fields:BTreeMap<&'a str, Node<'a>,
}

impl<'a> Map<'a> {
    fn parse( cur:&'a mut lexer::stream_lexer::Cursor, ends_with:char ) -> Result<Map, String>{
        let mut params=BTreeMap::new();

        loop {
            match try!(Lexeme::next( cur )) {
                Lexeme::String( paramName ) => {
                    let line=cur.line;

                    if try!(Lexeme::next( cur )) != Lexeme::Set {
                        return Err( format!("Line {} : Expected : or =", line ));
                    }

                    let paramValue=match try!(Lexeme::next( cur )) {
                        Lexeme::Bracket( '{') =>
                            ParameterValue::Map( try!(Map::parse( cur, '}', line )) ),
                        Lexeme::Bracket( '[') =>
                            ParameterValue::List( try!(List::parse( cur, line )) ),
                        Lexeme::String( s ) =>
                            ParameterValue::String{line:line, value:s },
                        Lexeme::NewLine | Lexeme::Comma | Lexeme::EOF | Lexeme::Bracket('}') =>
                            return Err(format!("Line {} : Value of parameter has been missed",line)),
                        _=>
                            return Err(format!("Line {} : Expected string(\"..\"), list([...]), map(_.._)",line)),
                    };

                    match params.entry( paramName ) {
                        Vacant( entry ) => {entry.insert( paramValue );},
                        Occupied( e ) => return Err(format!("Line {} : Parameter has been declarated before", line)),
                    }

                    match try!(Lexeme::next( cur )){
                        Lexeme::EOF => {
                            if endsWith!='\0' {
                                return Err(format!("Line {} : Expected _, but EOF was found",line));
                            }

                            break;
                        },
                        Lexeme::Bracket('}') => {
                            if endsWith!='}' {
                                return Err(format!("Line {} : Expected EOF, but _ was found",line));
                            }

                            break;
                        },
                        Lexeme::NewLine | Lexeme::Comma => {},
                        _=>return Err(format!("Line {} : Expected new line or , or {}", line, endsWith)),
                    }
                },
                Lexeme::NewLine => {},
                Lexeme::EOF => break,
                _=>return Err(format!("Line {} : Expected parameter name",cur.line)),
            }
        }

        Ok(
            Map{
                line:mapLine,
                params:params,
            }
        )
    }
