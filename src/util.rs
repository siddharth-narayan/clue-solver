fn next_player(turn: u32, total_players: u32) -> u32 {
    return (turn + 1) % (total_players);
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn recalculate_remaining_cards(
    players: &Vec<Player>,
    held_cards: &HashSet<Card>,
) -> (HashSet<Card>, HashSet<Card>, HashSet<Card>) {
    let remaining_people =
        players
            .iter()
            .fold(Card::all_people(), |mut remaining_people, player| {
                remaining_people
                    .retain(|card| !player.held_cards.contains(card) && !held_cards.contains(card));
                remaining_people
            });

    let remaining_weapons =
        players
            .iter()
            .fold(Card::all_weapons(), |mut remaining_weapons, player| {
                remaining_weapons
                    .retain(|card| !player.held_cards.contains(card) && !held_cards.contains(card));
                remaining_weapons
            });

    let remaining_locations =
        players
            .iter()
            .fold(Card::all_locations(), |mut remaining_locations, player| {
                remaining_locations
                    .retain(|card| !player.held_cards.contains(card) && !held_cards.contains(card));
                remaining_locations
            });

    (remaining_people, remaining_weapons, remaining_locations)
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

    fn as_hashset(&self) -> HashSet<Card> {
        let mut set = HashSet::new();

        set.insert(Card::Person(self.person));
        set.insert(Card::Weapon(self.weapon));
        set.insert(Card::Location(self.location));

        set
    }
}