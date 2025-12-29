use inquire_derive::Selectable;
use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Selectable)]
pub enum Person {
    White,
    Scarlett,
    Green,
    Mustard,
    Peacock,
    Plum,
}

impl Person {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::White,
            Self::Scarlett,
            Self::Green,
            Self::Mustard,
            Self::Peacock,
            Self::Plum,
        ]
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::White => write!(f, "White"),
            Self::Scarlett => write!(f, "Scarlett"),
            Self::Green => write!(f, "Green"),
            Self::Mustard => write!(f, "Mustard"),
            Self::Peacock => write!(f, "Peacock"),
            Self::Plum => write!(f, "Plum"),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Selectable)]

pub enum Weapon {
    Candlestick,
    Dagger,
    LeadPipe,
    Revolver,
    Rope,
    Wrench,
}
impl Weapon {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Candlestick,
            Self::Dagger,
            Self::LeadPipe,
            Self::Revolver,
            Self::Rope,
            Self::Wrench,
        ]
    }
}
impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Candlestick => write!(f, "Candlestick"),
            Self::Dagger => write!(f, "Dagger"),
            Self::LeadPipe => write!(f, "Lead Pipe"),
            Self::Revolver => write!(f, "Revolver"),
            Self::Rope => write!(f, "Rope"),
            Self::Wrench => write!(f, "Wrench"),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Selectable)]
pub enum Location {
    Kitchen,
    Ballroom,
    Conservatory,
    DiningRoom,
    BilliardRoom,
    Library,
    Lounge,
    Hall,
    Study,
}

impl Location {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Kitchen,
            Self::Ballroom,
            Self::Conservatory,
            Self::DiningRoom,
            Self::BilliardRoom,
            Self::Library,
            Self::Lounge,
            Self::Hall,
            Self::Study,
        ]
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Kitchen => write!(f, "Kitchen"),
            Self::Ballroom => write!(f, "Ballroom"),
            Self::Conservatory => write!(f, "Conservatory"),
            Self::DiningRoom => write!(f, "Dining Room"),
            Self::BilliardRoom => write!(f, "Billiard Room"),
            Self::Library => write!(f, "Library"),
            Self::Lounge => write!(f, "Lounge"),
            Self::Hall => write!(f, "Hall"),
            Self::Study => write!(f, "Study"),
        }
    }
}