use super::Location;

pub struct Text<'a>{
    pub location:Location,
    pub value:&'a str,
}

impl<'a> Text<'a>{
    pub fn new(location:Location, value:&'a str) -> Text<'a>{
        Text{
            location:location,
            value:value,
        }
    }
}
