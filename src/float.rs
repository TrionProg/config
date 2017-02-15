use lexer;

use Error;
use lexer::stream_lexer::Lexeme;
use CursorExt;
use Location;

pub struct Float{
    pub location:Location,
    pub value:f64,
}

impl Float{
    pub fn parse<'a>( cur:& mut lexer::stream_lexer::Cursor<'a>, is_negative:bool, ceil_part:i64, location:Location ) -> Result<Float, Error<'a>>{
        let fract_part=match cur.next_lexeme()? {
            Lexeme::Number( value ) => value,
            _ => return Err( Error::UnexpectedLexeme(cur.get_line(),cur.lex.clone(),"fractional part part") ),
        };

        let fract_divider={
            let mut fract_divider=1;
            let mut fract_value=fract_part;

            while fract_value>0 {
                fract_divider*=10;
                fract_value/=10;
            }

            fract_divider as f64
        };

        let result_float=match is_negative{
            true => -(ceil_part as f64 + (fract_part as f64)/fract_divider),
            false => (ceil_part as f64 + (fract_part as f64)/fract_divider),
        };

        cur.next_lexeme()?;

        Ok(
            Float{
                location:location,
                value:result_float,
            }
        )
    }
}
