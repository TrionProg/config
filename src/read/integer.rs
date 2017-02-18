use super::Location;

pub struct Integer{
    pub location:Location,
    pub value:i64,
}

impl Integer{
    pub fn new(location:Location, value:i64) -> Integer{
        Integer{
            location:location,
            value:value,
        }
    }
}
