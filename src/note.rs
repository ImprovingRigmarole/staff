//! Formatted notes

use crate::Natural;
use core::{
    fmt::{self, Write},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Accidental {
    Natural,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    pub fn is_natural(self) -> bool {
        match self {
            Self::Natural => true,
            _ => false,
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Accidental::Natural => f.write_char('♮'),
            Accidental::Flat => f.write_char('♭'),
            Accidental::DoubleFlat => f.write_str("𝄫"),
            Accidental::Sharp => f.write_char('♯'),
            Accidental::DoubleSharp => f.write_str("𝄪"),
        }
    }
}

#[cfg(feature = "ui")]
impl<'a> dioxus::prelude::IntoAttributeValue<'a> for Accidental {
    fn into_value(
        self,
        _bump: &'a dioxus::core::exports::bumpalo::Bump,
    ) -> dioxus::core::AttributeValue<'a> {
        dioxus::core::AttributeValue::Int(self as u8 as _)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    pub natural: Natural,
    pub accidental: Accidental,
}

impl Note {
    pub const C: Self = Self::new(Natural::C, Accidental::Natural);
    pub const D: Self = Self::new(Natural::D, Accidental::Natural);
    pub const E: Self = Self::new(Natural::E, Accidental::Natural);
    pub const F: Self = Self::new(Natural::F, Accidental::Natural);
    pub const G: Self = Self::new(Natural::G, Accidental::Natural);
    pub const A: Self = Self::new(Natural::A, Accidental::Natural);
    pub const B: Self = Self::new(Natural::B, Accidental::Natural);

    pub const fn new(natural: Natural, accidental: Accidental) -> Self {
        Self {
            natural,
            accidental,
        }
    }

    pub const fn flat(natural: Natural) -> Self {
        Self::new(natural, Accidental::Flat)
    }

    pub const fn double_flat(natural: Natural) -> Self {
        Self::new(natural, Accidental::DoubleFlat)
    }

    pub const fn sharp(natural: Natural) -> Self {
        Self::new(natural, Accidental::Sharp)
    }

    pub const fn double_sharp(natural: Natural) -> Self {
        Self::new(natural, Accidental::DoubleSharp)
    }
}

impl From<Natural> for Note {
    fn from(natural: Natural) -> Self {
        Self::new(natural, Accidental::Natural)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.natural, self.accidental)
    }
}

impl FromStr for Note {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let natural: Natural = if let Some(c) = chars.next() {
            c.try_into().unwrap()
        } else {
            return Err(Error::Empty);
        };

        let accidental = match chars.next() {
            Some('b') => match chars.next() {
                Some('b') => Accidental::DoubleFlat,
                Some(c) => return Err(c.into()),
                None => Accidental::Flat,
            },
            Some('#') => match chars.next() {
                Some('#') => Accidental::DoubleSharp,
                Some(c) => return Err(c.into()),
                None => Accidental::Sharp,
            },
            Some(c) => return Err(c.into()),
            None => Accidental::Natural,
        };

        Ok(Self::new(natural, accidental))
    }
}

#[derive(Debug)]
pub enum Error {
    Empty,
    Invalid(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("Empty note input"),
            Self::Invalid(c) => write!(f, "Invalid character `{}`", c),
        }
    }
}

impl From<char> for Error {
    fn from(c: char) -> Self {
        Self::Invalid(c)
    }
}
