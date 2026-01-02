

use std::collections::HashSet;
use crate::Card;
use inquire::Text;
use inquire::prompt_u32;
use crate::Guess;

pub struct Player {
    pub id: u32,
    pub name: String,
    pub card_count: u32,
    pub potentially_held_card_sets: Vec<HashSet<Card>>, // A potential set is a set where one of the cards is held by the player
    pub shown_cards: HashSet<Card>,                     // Cards we have shown this player before
    pub held_cards: HashSet<Card>,                      // Cards we know for sure they hold
    pub unheld_cards: HashSet<Card>,                    // Cards we know for sure they don't hold
}

impl Player {
    pub fn new_from_prompt(id: u32) -> Self {
        let name: String = Text::prompt("What is this player's name?".into()).unwrap();
        let card_count = prompt_u32("How many cards does this player have?").unwrap();
        Player {
            id: id,
            name: name,
            card_count: card_count,
            potentially_held_card_sets: Vec::new(),
            shown_cards: HashSet::new(),
            held_cards: HashSet::new(),
            unheld_cards: HashSet::new(),
        }
    }

    pub fn update_held(&mut self, card: &Card) {
        self.held_cards.insert(*card);
    }

    // pub fn prune_potential_sets(&mut self) {
    //     self.potentially_held_card_sets = self.potentially_held_card_sets.into_iter().filter_map(|set| {
    //         set.remove(self.unheld_cards);

    //         if set.len() == 1 {
    //             let held = set.into_iter().nth(0).unwrap();

    //             self.update_held(card);
    //             return None
    //         }
    //         Ok(set)
    //     });
    // }
    pub fn add_potential_set(&mut self, g: Guess) {
        self.potentially_held_card_sets.push(g.as_hashset());
    }
   
    pub fn update_unheld(&mut self, c: Card) {
        self.unheld_cards.insert(c);

        // // Infer: If a potential set is of size 1, then the card is necessarily held by the player
        // self.potentially_held_card_sets
        //     .iter_mut()
        //     .filter(|card_set| {
        //         if card_set.len() == 1 {
        //             let iter = card_set.iter();
        //             self.held_cards.insert(*iter.last().unwrap());
        //             return false;
        //         }

        //         return true;
        //     });
    }

    pub fn update_unheld_set(&mut self, c: &HashSet<Card>) {
        self.unheld_cards.extend(c);
    }

    pub fn display(&self) {
        println!("{}: (Player {})", self.name, self.id);
        println!("Holds these cards:");

        self.held_cards.iter().for_each(|card| {
            println!("{}", card);
        });

        println!("Is guaranteed to not hold these cards:");

        self.unheld_cards.iter().for_each(|card| {
            println!("{}", card);
        });

        println!("We've shown these cards to them:");

        self.shown_cards.iter().for_each(|card| {
            println!("{}", card);
        });
    }
}
