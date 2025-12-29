mod cards;

use std::collections::HashSet;
use std::fmt::{Display, Formatter, Pointer};
use std::hash::Hash;

use inquire::prompt_u32;
use inquire::{CustomType, InquireError, error::InquireResult, ui::RenderConfig};
use inquire::{Text, prompt_confirmation};
use inquire_derive::Selectable;

use cards::{Location, Person, Weapon};

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> InquireResult<()> {
    // let inquire_config = RenderConfig::default_colored().with_prompt_prefix( "".into());

    // Example using single select
    println!("Select all cards you have");
    let held_cards = Card::multi_select("Cards: ");

    held_cards.iter().for_each(|a| println!("{}", a));

    let player_count = prompt_u32("How many players other than you are there?").unwrap();

    let mut players: Vec<Player> = Vec::new();
    for i in 0..player_count {
        println!("For player {}, (you're player 0):", i + 1);
        let mut new_player = Player::new_from_prompt(i + 1);

        new_player.unheld_cards.extend(&held_cards); // Infer: We know they cant hold any cards we hold
        players.push(new_player);
    }

    let mut turn = prompt_u32("Who's turn is first?").unwrap();
    println!("Let the games begin!");

    loop {
        clear_terminal();
        println!("It's player {}'s turn. Enter their guess:", turn);
        let new_guess = Guess::new_from_prompt();

        let mut currently_showing_player = (turn + 1) % (player_count + 1);
        loop {
            let prompt = format!(
                "Does player {} have one of these cards?",
                currently_showing_player
            );

            if prompt_confirmation(prompt).unwrap() {
                if (turn == 0) {
                    // Infer: We are directly told (not really inferred)
                    let card = Card::select("What card did the player show you?");
                    let p: &mut Player = players.get_mut(currently_showing_player as usize).unwrap();
                    p.held_cards.insert(card);
                } else {
                    // Infer: Some player has one of these cards -- We can build a set of potential cards, and find the similarities between shows
                    // Maybe it's something like the union of all pairwise intersection of guess cards?
                }

                break;
            } else {
                // Infer: This person doesn't have any of these cards
                let p: &mut Player = players.get_mut(currently_showing_player as usize).unwrap();

                p.unheld_cards.insert(Card::Person(new_guess.person));
                p.unheld_cards.insert(Card::Weapon(new_guess.weapon));
                p.unheld_cards.insert(Card::Location(new_guess.location));
            }

            currently_showing_player += 1;
            currently_showing_player %= player_count + 1;

            if currently_showing_player == turn {
                break;
            }

        }

        players.iter().for_each(|p| {
            p.display();
        });

        turn += 1;
        turn %= player_count + 1;
    }

    Ok(())
}

struct Player {
    pub id: u32,
    pub name: String,
    pub card_count: u32,
    pub shown_cards: HashSet<Card>, // Cards we have shown this player before
    pub held_cards: HashSet<Card>,  // Cards we know for sure they hold
    pub unheld_cards: HashSet<Card>, // Cards we know for sure they don't hold
}

impl Player {
    pub fn new_from_prompt(id: u32) -> Self {
        let name: String = Text::prompt("What is this player's name?".into()).unwrap();
        let card_count = prompt_u32("How many cards does this player have?").unwrap();
        Player {
            id: id,
            name: name,
            card_count: card_count,
            shown_cards: HashSet::new(),
            held_cards: HashSet::new(),
            unheld_cards: HashSet::new(),
        }
    }

    pub fn display(&self) {
        println!("Player {} ({})", id, name)
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Guess {
    person: Person,
    weapon: Weapon,
    location: Location,
}

impl Guess {
    fn new_from_prompt() -> Self {
        let person = Person::select("Enter the person for this guess".into())
            .prompt()
            .unwrap();
        let weapon = Weapon::select("Enter the weapon for this guess".into())
            .prompt()
            .unwrap();
        let location = Location::select("Enter the location for this guess".into())
            .prompt()
            .unwrap();

        Guess {
            person: person,
            weapon: weapon,
            location: location,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]

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

    fn multi_select(prompt: &str) -> HashSet<Card> {
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
