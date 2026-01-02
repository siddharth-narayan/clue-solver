use inquire_derive::Selectable;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]

pub enum Card {
    Person(Person),
    Weapon(Weapon),
    Location(Location),
}

impl Card {
    pub fn all_people() -> HashSet<Card> {
        Person::all_variants_hashset()
            .into_iter()
            .map(|p| Self::Person(p))
            .collect::<HashSet<Card>>()
    }

    pub fn all_weapons() -> HashSet<Card> {
        Weapon::all_variants_hashset()
            .into_iter()
            .map(|p| Self::Weapon(p))
            .collect::<HashSet<Card>>()
    }

    pub fn all_locations() -> HashSet<Card> {
        Location::all_variants_hashset()
            .into_iter()
            .map(|p| Self::Location(p))
            .collect::<HashSet<Card>>()
    }

    pub fn all_cards() -> Vec<Card> {
        let mut cards = Vec::new();

        for person in Person::all_variants() {
            cards.push(Card::Person(person));
        }

        for weapon in Weapon::all_variants() {
            cards.push(Card::Weapon(weapon));
        }

        for location in Location::all_variants() {
            cards.push(Card::Location(location));
        }

        cards
    }

    pub fn all_cards_hashset() -> HashSet<Card> {
        Self::all_cards().into_iter().collect::<HashSet<Self>>()
    }

    pub fn select(prompt: &str) -> Card {
        inquire::Select::<Card>::new(prompt, Card::all_cards())
            .prompt()
            .unwrap()
    }

    pub fn multi_select(prompt: &str) -> HashSet<Card> {
        let cards = inquire::MultiSelect::new(prompt, Card::all_cards())
            .prompt()
            .unwrap();

        let mut card_set = HashSet::new();
        cards.into_iter().for_each(|card| {
            card_set.insert(card);
        });

        card_set
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Card::Person(p) => p.fmt(f),
            Card::Weapon(w) => w.fmt(f),
            Card::Location(l) => l.fmt(f),
        }
    }
}


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

    pub fn all_variants_hashset() -> HashSet<Self> {
        Self::all_variants().into_iter().collect::<HashSet<Self>>()
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

    pub fn all_variants_hashset() -> HashSet<Self> {
        Self::all_variants().into_iter().collect::<HashSet<Self>>()
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

    pub fn all_variants_hashset() -> HashSet<Self> {
        Self::all_variants().into_iter().collect::<HashSet<Self>>()
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
