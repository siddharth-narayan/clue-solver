use inquire_derive::Selectable;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Selectable)]
pub enum Person {
    White,
    Scarlett,
    Green,
    Mustard,
    Brown,
}

impl Person {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::White,
            Self::Scarlett,
            Self::Green,
            Self::Mustard,
            Self::Brown,
        ]
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Person::White => write!(f, "White"),
            Person::Scarlett => write!(f, "Scarlett"),
            Person::Green => write!(f, "Green"),
            Person::Mustard => write!(f, "Mustard"),
            Person::Brown => write!(f, "Brown"),
        }
    }
}

#[derive(Debug, Copy, Clone, Selectable)]

pub enum Weapon {
    White,
    Scarlett,
    Green,
    Mustard,
    Brown,
}
impl Weapon {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::White,
            Self::Scarlett,
            Self::Green,
            Self::Mustard,
            Self::Brown,
        ]
    }
}
impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Weapon::White => write!(f, "White"),
            Weapon::Scarlett => write!(f, "Scarlett"),
            Weapon::Green => write!(f, "Green"),
            Weapon::Mustard => write!(f, "Mustard"),
            Weapon::Brown => write!(f, "Brown"),
        }
    }
}

#[derive(Debug, Copy, Clone, Selectable)]
pub enum Location {
    White,
    Scarlett,
    Green,
    Mustard,
    Brown,
}
impl Location {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::White,
            Self::Scarlett,
            Self::Green,
            Self::Mustard,
            Self::Brown,
        ]
    }
}
impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Location::White => write!(f, "White"),
            Location::Scarlett => write!(f, "Scarlett"),
            Location::Green => write!(f, "Green"),
            Location::Mustard => write!(f, "Mustard"),
            Location::Brown => write!(f, "Brown"),
        }
    }
}
