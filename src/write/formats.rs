
pub enum SplitElementsWith{
    Space,
    Comma,
}

impl SplitElementsWith{
    pub fn get_str(&self) -> &'static str{
        match *self{
            SplitElementsWith::Space => " ",
            SplitElementsWith::Comma => ",",
        }
    }
}


pub enum SplitFieldsWith{
    Space,
    Comma,
    Semicolon,
}

impl SplitFieldsWith{
    pub fn get_str(&self) -> &'static str{
        match *self{
            SplitFieldsWith::Space => " ",
            SplitFieldsWith::Comma => ",",
            SplitFieldsWith::Semicolon => ";",
        }
    }
}


pub enum AssignmentSymbal{
    Eq,
    Colon,
}

impl AssignmentSymbal{
    pub fn get_str(&self) -> &'static str{
        match *self{
            AssignmentSymbal::Eq => " = ",
            AssignmentSymbal::Colon => " : ",
        }
    }
}
