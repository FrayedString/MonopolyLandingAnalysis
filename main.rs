
use futures::executor::block_on;
use rand::{Rng, thread_rng,};
use rand::rngs::ThreadRng;
use std::collections::{HashMap, VecDeque};
use core::cell::Cell;

//Represents the move action to be taken when a Chance or community chest card is drawn
type DrawnCardAction = fn(u8) -> Option<u8>;


struct BoardSpace {
    name : String,
    landed_count : Cell<u32>
}


struct Player
{
    name: String,
    current_space: Cell<u8>
}



#[derive(PartialEq)]
struct GameActionCard
{
    text: String,
    move_action: DrawnCardAction
}


struct CommunityChestDeck
{
    deck: VecDeque<GameActionCard>,
    dealt: Vec<GameActionCard>
}

impl CommunityChestDeck {
    fn new() -> Self {
        let mut deck = Self { deck: VecDeque::<GameActionCard>::with_capacity(16), dealt: Vec::<GameActionCard>::with_capacity(16) };

        //CARDS FROM: https://www.monopolyland.com/list-monopoly-chance-community-chest-cards/

        //Initialize cards into the Dealt vec, then shuffle on use
        deck.dealt.push(GameActionCard { text: String::from("Advance to GO"), move_action: |_| Some(0) });
        deck.dealt.push(GameActionCard { text: String::from("Bank error in your favor. Collect $200"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Doctor’s fee. Pay $50"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("From sale of stock you get $50"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Get Out of Jail Free"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Go to Jail. Go directly to jail, do not pass Go, do not collect $200"), move_action: |_| Some(10) });
        deck.dealt.push(GameActionCard { text: String::from("Holiday fund matures. Receive $100"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Income tax refund. Collect $20"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("It is your birthday. Collect $10 from every player"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Life insurance matures. Collect $100"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Pay hospital fees of $100"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Pay school fees of $50"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Receive $25 consultancy fee"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("You are assessed for street repair. $40 per house. $115 per hotel"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("You have won second prize in a beauty contest. Collect $10"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("You inherit $100"), move_action: |_| None });

        deck
    }
}


struct ChanceDeck
{
    deck: VecDeque<GameActionCard>,
    dealt: Vec<GameActionCard>
}

impl ChanceDeck {
    fn new () -> Self {
        let mut deck = Self { deck: VecDeque::<GameActionCard>::with_capacity(16), dealt: Vec::<GameActionCard>::with_capacity(16) };

        //CARDS FROM: https://www.monopolyland.com/list-monopoly-chance-community-chest-cards/

        //Initialize cards into the Dealt vec, then shuffle on use
        deck.dealt.push(GameActionCard { text: String::from("Advance to Boardwalk"), move_action: |_| Some(39) });
        deck.dealt.push(GameActionCard { text: String::from("Advance to Go (Collect $200)"), move_action: |_| Some(0) });
        deck.dealt.push(GameActionCard { text: String::from("Advance to Illinois Avenue. If you pass Go, collect $200"), move_action: |_| Some(24) });
        deck.dealt.push(GameActionCard { text: String::from("Advance to St. Charles Place. If you pass Go, collect $200"), move_action: |_| Some(11) });
        deck.dealt.push(GameActionCard { text: String::from("Advance to the nearest Railroad. If unowned, you may buy it from the Bank. If owned, pay owner twice the rental to which they are otherwise entitled"), move_action: |landed| {
            match landed {
                7 => Some(15),  //Pennsylvania Railroad
                22 => Some(25),  //B&O Railroad
                36 => Some(5),    //Reading Railroad
                _ => None
            }
        }});
        deck.dealt.push(GameActionCard { text: String::from("Advance to the nearest Railroad. If unowned, you may buy it from the Bank. If owned, pay owner twice the rental to which they are otherwise entitled"), move_action: |landed| {
            match landed {
                7 => Some(15),  //Pennsylvania Railroad
                22 => Some(25),  //B&O Railroad
                36 => Some(5),    //Reading Railroad
                _ => None
            }
        }});
        deck.dealt.push(GameActionCard { text: String::from("Advance token to nearest Utility. If unowned, you may buy it from the Bank. If owned, throw dice and pay owner a total ten times amount thrown."), move_action: |landed| {
            match landed {
                7 | 36 => Some(12),  //Electric Company
                22 => Some(28),  //Water Works
                _ => None
            }
        }});
        deck.dealt.push(GameActionCard { text: String::from("Bank pays you dividend of $50"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Get Out of Jail Free"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Go Back 3 Spaces"), move_action: |landed| Some(landed - 3) });
        deck.dealt.push(GameActionCard { text: String::from("Go to Jail. Go directly to jail, do not pass Go, do not collect $200"), move_action: |_| Some(10) });
        deck.dealt.push(GameActionCard { text: String::from("Make general repairs on all your property. For each house pay $25. For each hotel pay $100"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Speeding fine $15"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Take a trip to Reading Railroad. If you pass Go, collect $200"), move_action: |_| Some(5) });
        deck.dealt.push(GameActionCard { text: String::from("You have been elected Chairman of the Board. Pay each player $50"), move_action: |_| None });
        deck.dealt.push(GameActionCard { text: String::from("Your building loan matures. Collect $150"), move_action: |_| None });

        deck
    }
}




fn main() {
    
    let game_board = initialize_board();

    let mut community_chest = CommunityChestDeck::new();
    let mut chance = ChanceDeck::new();

    //Spaces 0-39 where 0=go and 39=boardwalk
    //let spaces: [Mutex<i32>; 40] = array_init::array_init(|_i: usize| (Cell::<u32>::from(0)));

    let work = start_work(4, 30, &game_board, &mut community_chest, &mut chance);
    block_on(work);


    for i in 0..game_board.len() {
        let idx = i as u8;

        let space = game_board.get(&idx);

        if space.is_some() {
            let real_space = space.unwrap();

            println!("{}|{}", real_space.landed_count.get(), real_space.name)
        }
        else {
            println!("out of range");
        }
    }
}


async fn start_work(player_cnt: u32, turns: u32, board: &HashMap<u8, BoardSpace>, community_chest: &mut CommunityChestDeck, chance: &mut ChanceDeck) {
    let players: Vec<_> = (1..=player_cnt).map(|i| Player { name: format!("Player {}", i), current_space: Cell::<u8>::from(0) }).collect();
    let mut rng = thread_rng();

    for _turn in 1..turns {
        for player in &players {
            take_player_turn(player, &mut rng, board, community_chest, chance);
        }
    }
}

fn take_player_turn(player: &Player, rng: &mut ThreadRng, board: &HashMap<u8, BoardSpace>, community_chest: &mut CommunityChestDeck, chance: &mut ChanceDeck) {
    let dice1 = rng.gen_range(1..7);
    let dice2 = rng.gen_range(1..7);


    println!("{} Rolled {} and {} totaling {}", player.name, dice1, dice2, dice1+dice2);


    let mut landed_space = player.current_space.get() + dice1 + dice2;

    if landed_space > 39 {
        landed_space -= 40;
    }
    

    log_space_landed(board, landed_space, player);

    
    let draw_card_option = match landed_space {
        2 | 17 | 33 => Some(draw_community_chest(community_chest)),
        7 | 22 | 36 => Some(draw_chance(chance)),
        _ => None
    };

    let drawn_card = match draw_card_option {
        Some(card_option) => card_option,
        None => None
    };
    
    if drawn_card.is_some() {
        //PROCESS THE CARD
        let card = drawn_card.unwrap();
        let action = (card.move_action)(landed_space);

        match action {
            None => println!("{} Drew Card {}", player.name, card.text),
            Some(new_space)=> {
                println!("{} Drew Card {}", player.name, card.text);
                log_space_landed(board, new_space, player);
            }
        };
    }
}



fn log_space_landed(board: &HashMap<u8, BoardSpace>, landed_space: u8, player: &Player) {
    //Log the landing
    let space = board.get(&landed_space).unwrap();

    let mut cur_land_count = space.landed_count.get();
    cur_land_count += 1;
    space.landed_count.set(cur_land_count);

    player.current_space.set(landed_space);

    println!("{} is on {}", player.name, space.name);
}





fn draw_community_chest(deck: &mut CommunityChestDeck) -> Option<&GameActionCard> {
    //Shuffle if needed
    if deck.deck.len() == 0 {
        let mut rng = thread_rng();

        while deck.dealt.len() > 0 {
            deck.deck.push_back(deck.dealt.swap_remove(rng.gen_range(0..deck.dealt.len())));
        }
    }
    
    //Draw
    let next_card = deck.deck.pop_front();
    
    if next_card == None {
        return None;
    }

    deck.dealt.push(next_card.unwrap());
    
    //This should return the card we just pushed.  (assuming single-threaded)
    deck.dealt.last()
}



fn draw_chance(deck: &mut ChanceDeck) -> Option<&GameActionCard> {
    //Shuffle if needed
    if deck.deck.len() == 0 {
        let mut rng = thread_rng();

        while deck.dealt.len() > 0 {
            deck.deck.push_back(deck.dealt.swap_remove(rng.gen_range(0..deck.dealt.len())));
        }
    }
    
    //Draw
    let next_card = deck.deck.pop_front();
    
    if next_card == None {
        return None;
    }

    deck.dealt.push(next_card.unwrap());
    
    //This should return the card we just pushed.  (assuming single-threaded here)
    deck.dealt.last()
}



fn initialize_board() -> HashMap::<u8, BoardSpace> {
    let mut space_defs = HashMap::<u8, BoardSpace>::new();

    space_defs.insert(0, BoardSpace {name: String::from("Go"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(1, BoardSpace {name: String::from("Mediterranean Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(2, BoardSpace {name: String::from("Community Chest"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(3, BoardSpace {name: String::from("Baltic Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(4, BoardSpace {name: String::from("Income Tax"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(5, BoardSpace {name: String::from("Reading Railroad"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(6, BoardSpace {name: String::from("Oriental Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(7, BoardSpace {name: String::from("Chance"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(8, BoardSpace {name: String::from("Vermont Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(9, BoardSpace {name: String::from("Connecticut Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(10, BoardSpace {name: String::from("Jail"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(11, BoardSpace {name: String::from("St. Charles Place"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(12, BoardSpace {name: String::from("Electric Company"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(13, BoardSpace {name: String::from("States Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(14, BoardSpace {name: String::from("Virginia Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(15, BoardSpace {name: String::from("Pennsylvania Railroad"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(16, BoardSpace {name: String::from("St. James Place"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(17, BoardSpace {name: String::from("Community Chest"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(18, BoardSpace {name: String::from("Tennessee Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(19, BoardSpace {name: String::from("New York Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(20, BoardSpace {name: String::from("Free Parking"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(21, BoardSpace {name: String::from("Kentucky Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(22, BoardSpace {name: String::from("Chance"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(23, BoardSpace {name: String::from("Indiana Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(24, BoardSpace {name: String::from("Illinois Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(25, BoardSpace {name: String::from("B & O Railroad"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(26, BoardSpace {name: String::from("Atlantic Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(27, BoardSpace {name: String::from("Ventnor Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(28, BoardSpace {name: String::from("Waterworks"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(29, BoardSpace {name: String::from("Marvin Gardens"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(30, BoardSpace {name: String::from("Go to Jail"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(31, BoardSpace {name: String::from("Pacific Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(32, BoardSpace {name: String::from("North Carolina Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(33, BoardSpace {name: String::from("Community Chest"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(34, BoardSpace {name: String::from("Pennsylvania Avenue"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(35, BoardSpace {name: String::from("Short Line Railroad"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(36, BoardSpace {name: String::from("Chance"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(37, BoardSpace {name: String::from("Park Place"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(38, BoardSpace {name: String::from("Luxury Tax"), landed_count: Cell::<u32>::from(0)});
    space_defs.insert(39, BoardSpace {name: String::from("Boardwalk"), landed_count: Cell::<u32>::from(0)});

    space_defs
}