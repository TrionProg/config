use std;
use super::Error;

pub struct Writer<'a,W:std::io::Write+'a>{
    write_to:&'a mut W,
    tabulation:String,
}

impl<'a,W:std::io::Write> Writer<'a,W>{
    pub fn new(write_to:&'a mut W) ->  Writer<'a,W>{
        Writer{
            write_to:write_to,
            tabulation:String::new(),
        }
    }

    pub fn write_str(&mut self, text:&str) -> Result<(),Error>{
        match self.write_to.write_all(text.as_bytes()) {
            Ok ( _ ) => Ok(()),
            Err( e ) => Err( Error::WriteIOResult(e) ),
        }
    }

    pub fn write_text(&mut self, text:&str) -> Result<(),Error>{
        if text.contains(char::is_whitespace) | text.contains('.') {//TODO: replace to only alpha|digit|_ , add shielding
            self.write_str( format!("\"{}\"",text).as_str() )
        }else{
            self.write_str(text)
        }
    }

    pub fn write_tab(&mut self) -> Result<(),Error>{
        match self.write_to.write_all(self.tabulation.as_bytes()) {
            Ok ( _ ) => Ok(()),
            Err( e ) => Err( Error::WriteIOResult(e) ),
        }
    }

    pub fn inc_tab(&mut self) {
        self.tabulation.push_str("    ");
    }

    pub fn dec_tab(&mut self) {
        let new_length=self.tabulation.len()-4;
        self.tabulation.truncate(new_length);
    }
}
