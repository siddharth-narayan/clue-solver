mod cards;
mod player;
mod util;

use inquire::{error::InquireResult, prompt_confirmation, prompt_u32};

use cards::{Card, Person, Weapon};
use player::Player;
use util::{Guess, clear_terminal, next_player};

fn main() -> InquireResult<()> {
    // let inquire_config = RenderConfig::default_colored().with_prompt_prefix( "".into());

    // Example using single select
    println!("Select all cards you have");
    let held_cards = Card::multi_select("Cards: ");

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
        println!("It's player {}'s turn. Enter their guess:", turn);
        let new_guess = Guess::new_from_prompt();

        let mut currently_showing_player_idx = next_player(turn, player_count + 1); // 0 indexed in the player vec, so make sure it doesn't go over that length

        loop {
            let currently_showing_player: &mut Player = players
                .get_mut((currently_showing_player_idx - 1) as usize)
                .unwrap();

            let prompt = format!(
                "Does player {} have one of these cards?",
                currently_showing_player.id
            );

            if prompt_confirmation(prompt).unwrap() {
                if turn == 0 {
                    let card = Card::select("What card did the player show you?");

                    players
                        .iter_mut()
                        .nth((currently_showing_player_idx - 1) as usize)
                        .unwrap()
                        .update_held(&card);
                } else {
                    // Infer: Some player has one of these cards -- We can build a set of potential cards, and find the similarities between shows
                    // Maybe it's something like the union of all pairwise intersection of guess cards?

                    currently_showing_player.add_potential_set(new_guess);
                }

                break;
            } else {
                // Infer: This person doesn't have any of these cards

                currently_showing_player.update_unheld(Card::Person(new_guess.person));
                currently_showing_player.update_unheld(Card::Weapon(new_guess.weapon));
                currently_showing_player.update_unheld(Card::Location(new_guess.location));
            }

            currently_showing_player_idx += 1;
            currently_showing_player_idx %= player_count + 1;

            if currently_showing_player_idx == turn {
                break;
            }
        }

        clear_terminal();

        players.iter().for_each(|p| {
            p.display();
        });

        // let (remaining_people, remaining_weapons, remaining_locations) =
        //     recalculate_remaining_cards(&players, &held_cards);

        // println!("Remaining people:");
        // remaining_people
        //     .iter()
        //     .for_each(|person| println!("\t{}", person));

        // println!("Remaining weapons:");
        // remaining_weapons
        //     .iter()
        //     .for_each(|weapon| println!("\t{}", weapon));

        // println!("Remaining locations:");
        // remaining_locations
        //     .iter()
        //     .for_each(|location| println!("\t{}", location));

        turn = next_player(turn, player_count + 1);
    }
}
