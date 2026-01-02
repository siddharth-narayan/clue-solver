use crate::Card;
use crate::Person;
use crate::Player;
use crate::Weapon;
use crate::cards::Location;

use std::collections::HashSet;

pub fn next_player(turn: u32, total_players: u32) -> u32 {
    return (turn + 1) % (total_players);
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

// GPT-Generated :(
pub fn recalculate_remaining_cards(
    players: &[Player],
    held_cards: &HashSet<Card>,
) -> (HashSet<Card>, HashSet<Card>, HashSet<Card>) {
    let mut seen: HashSet<Card> = HashSet::new();

    seen.extend(held_cards.iter());

    seen.extend(players.iter().flat_map(|p| p.held_cards.iter()));

    let filter = |all: HashSet<Card>| all.into_iter().filter(|c| !seen.contains(c)).collect();

    (
        filter(Card::all_people()),
        filter(Card::all_weapons()),
        filter(Card::all_locations()),
    )
}

pub fn run_inference(players: &mut Vec<Player>, held_cards: &HashSet<Card>) {
    let mut seen_cards: HashSet<Card> = HashSet::new();
    seen_cards.extend(held_cards.iter());

    for i in 0..players.len() {
        let (left, rest) = players.split_at_mut(i);
        let (player, right) = rest.split_first_mut().unwrap();

        seen_cards.extend(player.held_cards.iter());

        // Infer: no player can hold a card held by another player
        for other_player in left.iter_mut().chain(right.iter_mut()) {
            other_player.update_unheld_set(&player.held_cards); // Infer
        }
    }

    // Infer: If a card is unheld, it's not held in our potential sets
    for player in players.iter_mut() {
        for card in player.unheld_cards.iter() {
            for card_set in player.potentially_held_card_sets.iter_mut() {
                card_set.remove(card);
            }
        }
    }

    // Infer: If there's a potential set with just one card, we must hold it
    for player in players.iter_mut() {
        player
            .potentially_held_card_sets
            .retain(|set| set.len() > 1);
    }

    // seen_cards.extend(players.iter().map(|player| player.held_cards))
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct Guess {
    pub person: Person,
    pub weapon: Weapon,
    pub location: Location,
}

impl Guess {
    pub fn new_from_prompt() -> Self {
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

    pub fn as_hashset(&self) -> HashSet<Card> {
        let mut set = HashSet::new();

        set.insert(Card::Person(self.person));
        set.insert(Card::Weapon(self.weapon));
        set.insert(Card::Location(self.location));

        set
    }
}
