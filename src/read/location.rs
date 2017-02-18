use std;
use lexer;

#[derive(Debug,Clone)]
pub struct Location{
    pub line:usize,
    pub column:usize,
}

impl Location{
    pub fn new( cursor:&lexer::stream_lexer::Cursor ) -> Location{
        Location{
            line:cursor.line_number,
            column:cursor.column,
        }
    }

    pub fn document_beginning() -> Location{
        Location{
            line:1,
            column:1,
        }
    }
}

impl std::fmt::Display for Location{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Line:{} col:{} ", self.line, self.column)
    }
}
