use std::{borrow::Borrow, fmt::Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct U3(u8);

impl Display for U3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Borrow<str> for U3 {
    fn borrow(&self) -> &str {
        match self.get() {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            _ => unreachable!(),
        }
    }
}

impl U3 {
    pub fn new<T>(value: T) -> Option<Self>
    where
        T: Into<u64>,
    {
        let value: u64 = value.into();
        if value <= 7 {
            Some(Self(value as u8))
        } else {
            None
        }
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for U3 {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 7 { Ok(Self(value)) } else { Err(()) }
    }
}
