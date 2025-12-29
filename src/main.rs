mod cards;

use std::fmt::{Display, Formatter};
use std::collections::HashSet;

use inquire::{CustomType, InquireError, error::InquireResult, ui::RenderConfig};
use inquire_derive::Selectable;

use cards::{Location, Person, Weapon};

fn main() -> InquireResult<()> {
    // let inquire_config = RenderConfig::default_colored().with_prompt_prefix( "".into());

    // Example using single select
    println!("Select all cards you have");
    let held_cards: Vec<Card> = Card::multi_select("Cards: ");

    held_cards.iter().for_each(|a| {println!("{}", a)} );


    let player_count: u32 = CustomType::<u32>::new("How many players other than you are there?").prompt()?;

    let players: Vec<Player> = Vec::new();
    for i in 0..player_count {
    }

    Ok(())
}

struct Player {
    held_cards: HashSet<Card>, // Cards we know for sure they hold
    unheld_cards: HashSet<Card> // Cards we know for sure they don't hold
}

impl Player {
    pub fn new() -> Self {
        Player {
            held_cards: HashSet::new(),
            unheld_cards: HashSet::new()
        }
    }
}

#[derive(Debug, Copy, Clone)]

enum Card {
    Person(Person),
    Weapon(Weapon),
    Location(Location),
}

impl Card {
    fn all_cards() -> Vec<Card> {
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

    fn select(prompt: &str) -> Card {
        inquire::Select::<Card>::new(prompt, Card::all_cards())
            .prompt()
            .unwrap()
    }

    fn multi_select(prompt: &str) -> Vec<Card>  {
        inquire::MultiSelect::new(prompt, Card::all_cards())
            .prompt()
            .unwrap()
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
