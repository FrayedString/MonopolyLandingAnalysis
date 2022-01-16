use std::collections::HashMap;

use rand::{thread_rng, prelude::ThreadRng, Rng};

mod player;
use player::Player;

mod game_cards;
use game_cards::{CardDecks};

mod game_board;
use game_board::{BoardSpace, SpaceActionEnum, CardDeckEnum};



pub async fn run_simulation(player_count: u32, turn_count: u32) {

    let mut players: Vec<_> = (1..=player_count).map(|i| Player::new(format!("Player {}", i).to_string())).collect();

    let mut rng = thread_rng();
    let mut game_board = game_board::initialize_game_board();

    let mut card_decks = CardDecks::new();

    for _turn in 1..=turn_count {
        for player in players.iter_mut() {
            take_player_turn(player, &mut rng, &mut game_board, &mut card_decks, 0);
        }
    }


    //Print the game summary:
    for i in 0..game_board.len() {
        let space = game_board.get(&i);

        if space.is_some() {
            let real_space = space.unwrap();

            println!("{}|{}", real_space.get_landed_count(), real_space.get_space_name(i as u8))
        }
        else {
            println!("out of range");
        }
    }
}






fn take_player_turn(player: &mut Player, rng: &mut ThreadRng, board: &mut HashMap<usize, Box<dyn BoardSpace>>, card_decks: &mut CardDecks, mut doubles_count: u8) {
    let dice1 = rng.gen_range(1..=6);
    let dice2 = rng.gen_range(1..=6);

    let mut landed_space = player.get_current_space() + dice1 + dice2;
    let mut roll_again = false;

    if dice1 == dice2 {
        doubles_count = doubles_count + 1;

        if doubles_count == 1 {
            println!("{} Rolled {} and {} totaling {}.  Doubles Once.", player.get_player_name(), dice1, dice2, dice1+dice2);
            roll_again = true;
        }
        else if doubles_count == 2 {
            println!("{} Rolled {} and {} totaling {}.  Doubles Twice.", player.get_player_name(), dice1, dice2, dice1+dice2);
            roll_again = true;
        }
        else if doubles_count == 3 {
            println!("{} Rolled {} and {} totaling {}.  Doubles Thrice. Go directly to jail, do not pass go, do not collect $200", player.get_player_name(), dice1, dice2, dice1+dice2);

            //Override the landed space to be the jail!
            landed_space = 10;
        }
    }
    else {
        println!("{} Rolled {} and {} totaling {}", player.get_player_name(), dice1, dice2, dice1+dice2);
    }

    //Handle "passing go"
    if landed_space > 39 {
        landed_space -= 40;
    }
    

    //Process any special space behaviors (go to jail, draw cards)
    loop {
        let space = board.get_mut(&(landed_space as usize)).unwrap();
        let space_action = space.increment_landed(player);

        landed_space = 
            match space_action {
                SpaceActionEnum::NoAction => break,
                SpaceActionEnum::DrawCard(deck) => {
                    let card = match deck {
                        CardDeckEnum::Chance => card_decks.get_chance_deck().draw_card().unwrap(),
                        CardDeckEnum::CommunityChest => card_decks.get_community_chest_deck().draw_card().unwrap()
                    };

                    println!("{} Drew Card {}", player.get_player_name(), card.get_card_text());

                    match (card.get_move_action())(landed_space) {
                        None => break,
                        Some(card_moved_space) => card_moved_space
                    }
                },
                SpaceActionEnum::MovePlayer(new_space) => new_space
            };
    }


    //Now that we've finalized our ending place, update the player's saved location
    player.set_current_space(landed_space);
    



    //Now that we're done processing the players turn, if they rolled doubles they need to take another turn
    if roll_again {
        //RECURSION!
        take_player_turn(player, rng, board, card_decks, doubles_count);
    }
}