struct Player {
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

    pub fn update_held(&mut self, card: Card) {
        self.held_cards.insert(card);
    }

    pub fn update_potential_sets(&mut self, mut players: &Vec<Player>, held_cards: HashSet<Card>, g: Guess) {
        // Update a potential set with a new guess
        let potential_set = g.as_hashset();

        let potential_set = players
            .iter_mut()
            .fold(potential_set, |mut remaining_cards, player| {
                remaining_cards
                    .retain(|card| !player.held_cards.contains(card) && !held_cards.contains(card));
                remaining_cards
            });

        // Infer: If other people hold 2 of the 3 cards in the guess, the shown card must be the remaining one
        if potential_set.len() == 1 {
            self.held_cards
                .insert(potential_set.into_iter().last().unwrap());
        }
    }
    pub fn update_unheld(&mut self, c: Card) {
        self.unheld_cards.insert(c);

        // Infer: If a card is unheld, it's not held in our potential sets, so we should update them
        self.potentially_held_card_sets
            .retain(|card_set| !card_set.contains(&c));

        // Infer: If a potential set is of size 1, then the card is necessarily held by the player
        self.potentially_held_card_sets
            .iter_mut()
            .filter(|card_set| {
                if card_set.len() == 1 {
                    let iter = card_set.iter();
                    self.held_cards.insert(*iter.last().unwrap());
                    return false;
                }

                return true;
            });
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
