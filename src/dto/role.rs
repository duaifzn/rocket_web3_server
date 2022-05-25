use std::fmt;

#[derive(Debug)]
pub enum Role{
    Admin,
    User,
    Visitor,
}

impl Role{
    pub fn from_u8(value: u8) ->Role{
        match value{
            0 => Role::Admin,
            1 => Role::User,
            2 => Role::Visitor,
            _ => Role::Visitor
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
