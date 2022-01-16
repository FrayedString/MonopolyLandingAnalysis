use std::collections::{VecDeque, HashMap};

use rand::{thread_rng, prelude::ThreadRng, Rng};


struct CardDecks 
{
    chance_deck: GameActionCardDeck,
    community_chest_deck: GameActionCardDeck,
}




pub struct GameSimulation
{
    players: Vec<Player>,
}

impl GameSimulation {
    pub fn new(player_count: u32) -> Self {
        let players: Vec<_> = (1..=player_count).map(|i| Player::new(format!("Player {}", i).to_string())).collect();
    
        GameSimulation {players }
    }


    pub async fn run_simulation(&mut self, turn_count: u32) {

        //Reset all player positions to start
        for i in 0..self.players.len() {
            self.players[i].set_current_space(0);
        }

        let mut rng = thread_rng();
        let mut game_board = initialize_game_board();

        let mut card_decks = CardDecks {
            chance_deck: init_chance(),
            community_chest_deck: init_community_chest()
        };

        for _turn in 1..=turn_count {
            for player in self.players.iter_mut() {
                take_player_turn(player, &mut rng, &mut game_board, &mut card_decks, 0);
            }
        }


        //Print the game summary:
        for i in 0..game_board.len() {
            let idx = i as u8;

            let space = game_board.get(&idx);

            if space.is_some() {
                let real_space = space.unwrap();

                println!("{}|{}", real_space.get_landed_count(), real_space.get_space_name())
            }
            else {
                println!("out of range");
            }
        }
    }
}



fn take_player_turn(player: &mut Player, rng: &mut ThreadRng, board: &mut HashMap<u8, Box<dyn BoardSpace>>, card_decks: &mut CardDecks, mut doubles_count: u8) {
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
        let space = board.get_mut(&landed_space).unwrap();
        let space_action = space.increment_landed(player);

        landed_space = 
            match space_action {
                SpaceActionEnum::NoAction => break,
                SpaceActionEnum::DrawCard(deck) => {
                    let card = match deck {
                        CardDeckEnum::Chance => card_decks.chance_deck.draw_card().unwrap(),
                        CardDeckEnum::CommunityChest => card_decks.community_chest_deck.draw_card().unwrap()
                    };

                    println!("{} Drew Card {}", player.get_player_name(), card.text);

                    match (card.move_action)(landed_space) {
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






fn init_community_chest() -> GameActionCardDeck {
    let mut comm_chest_cards: Vec<_> = Vec::<GameActionCard>::with_capacity(16);
    comm_chest_cards.push(GameActionCard { text: String::from("Advance to GO"), move_action: |_| Some(0) });
    comm_chest_cards.push(GameActionCard { text: String::from("Bank error in your favor. Collect $200"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Doctor’s fee. Pay $50"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("From sale of stock you get $50"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Get Out of Jail Free"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Go to Jail. Go directly to jail, do not pass Go, do not collect $200"), move_action: |_| Some(10) });
    comm_chest_cards.push(GameActionCard { text: String::from("Holiday fund matures. Receive $100"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Income tax refund. Collect $20"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("It is your birthday. Collect $10 from every player"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Life insurance matures. Collect $100"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Pay hospital fees of $100"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Pay school fees of $50"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("Receive $25 consultancy fee"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("You are assessed for street repair. $40 per house. $115 per hotel"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("You have won second prize in a beauty contest. Collect $10"), move_action: |_| None });
    comm_chest_cards.push(GameActionCard { text: String::from("You inherit $100"), move_action: |_| None });

    GameActionCardDeck::new(comm_chest_cards)
}



fn init_chance() -> GameActionCardDeck {
    let mut chance_cards: Vec<_> = Vec::<GameActionCard>::with_capacity(16);


    //CARDS FROM: https://www.monopolyland.com/list-monopoly-chance-community-chest-cards/

        //Initialize cards into the Dealt vec, then shuffle on use
        chance_cards.push(GameActionCard { text: String::from("Advance to Boardwalk"), move_action: |_| Some(39) });
        chance_cards.push(GameActionCard { text: String::from("Advance to Go (Collect $200)"), move_action: |_| Some(0) });
        chance_cards.push(GameActionCard { text: String::from("Advance to Illinois Avenue. If you pass Go, collect $200"), move_action: |_| Some(24) });
        chance_cards.push(GameActionCard { text: String::from("Advance to St. Charles Place. If you pass Go, collect $200"), move_action: |_| Some(11) });
        chance_cards.push(GameActionCard { text: String::from("Advance to the nearest Railroad. If unowned, you may buy it from the Bank. If owned, pay owner twice the rental to which they are otherwise entitled"), move_action: |landed| {
            match landed {
                7 => Some(15),  //Pennsylvania Railroad
                22 => Some(25),  //B&O Railroad
                36 => Some(5),    //Reading Railroad
                _ => None
            }
        }});
        chance_cards.push(GameActionCard { text: String::from("Advance to the nearest Railroad. If unowned, you may buy it from the Bank. If owned, pay owner twice the rental to which they are otherwise entitled"), move_action: |landed| {
            match landed {
                7 => Some(15),  //Pennsylvania Railroad
                22 => Some(25),  //B&O Railroad
                36 => Some(5),    //Reading Railroad
                _ => None
            }
        }});
        chance_cards.push(GameActionCard { text: String::from("Advance token to nearest Utility. If unowned, you may buy it from the Bank. If owned, throw dice and pay owner a total ten times amount thrown."), move_action: |landed| {
            match landed {
                7 | 36 => Some(12),  //Electric Company
                22 => Some(28),  //Water Works
                _ => None
            }
        }});
        chance_cards.push(GameActionCard { text: String::from("Bank pays you dividend of $50"), move_action: |_| None });
        chance_cards.push(GameActionCard { text: String::from("Get Out of Jail Free"), move_action: |_| None });
        chance_cards.push(GameActionCard { text: String::from("Go Back 3 Spaces"), move_action: |landed| Some(landed - 3) });
        chance_cards.push(GameActionCard { text: String::from("Go to Jail. Go directly to jail, do not pass Go, do not collect $200"), move_action: |_| Some(10) });
        chance_cards.push(GameActionCard { text: String::from("Make general repairs on all your property. For each house pay $25. For each hotel pay $100"), move_action: |_| None });
        chance_cards.push(GameActionCard { text: String::from("Speeding fine $15"), move_action: |_| None });
        chance_cards.push(GameActionCard { text: String::from("Take a trip to Reading Railroad. If you pass Go, collect $200"), move_action: |_| Some(5) });
        chance_cards.push(GameActionCard { text: String::from("You have been elected Chairman of the Board. Pay each player $50"), move_action: |_| None });
        chance_cards.push(GameActionCard { text: String::from("Your building loan matures. Collect $150"), move_action: |_| None });

        GameActionCardDeck::new(chance_cards)
}



struct Player
{
    name: String,
    current_space: u8
}

impl Player {
    fn new(name: String) -> Self {
        Player { name, current_space: 0 }
    }

    fn set_current_space (&mut self, space: u8) {
        self.current_space = space;
    }

    fn get_current_space(&self) -> u8 {
        self.current_space
    }

    fn get_player_name(&self) -> &str {
        &self.name
    }
}



//Represents the move action to be taken when a Chance or community chest card is drawn
type DrawnCardAction = fn(u8) -> Option<u8>;

#[derive(PartialEq)]
struct GameActionCard
{
    text: String,
    move_action: DrawnCardAction
}



struct GameActionCardDeck
{
    deck: VecDeque<GameActionCard>,
    dealt: Vec<GameActionCard>
}

impl GameActionCardDeck {
    fn new(cards: Vec::<GameActionCard>) -> Self {
        //Initialize cards into the Dealt vec, deck will shuffle on first use
        GameActionCardDeck { deck: VecDeque::<GameActionCard>::with_capacity(16), dealt: cards }
    }


    fn draw_card(&mut self) -> Option<&GameActionCard> {
        //Shuffle if needed
        if self.deck.len() == 0 {
            let mut rng = thread_rng();

            while self.dealt.len() > 0 {
                self.deck.push_back(self.dealt.swap_remove(rng.gen_range(0..self.dealt.len())));
            }   
        }

        //Draw
        let next_card = self.deck.pop_front();
        
        if next_card == None {
            return None;
        }


        self.dealt.push(next_card.unwrap());
    
        //This should return the card we just pushed.  (assuming single-threaded)
        self.dealt.last()
    }
}






enum CardDeckEnum {
    Chance,
    CommunityChest
}

enum SpaceActionEnum {
    NoAction,
    DrawCard(CardDeckEnum),
    MovePlayer(u8),
}




trait BoardSpace {
    fn get_space_name(&self) -> &str;
    fn get_landed_count(&self) -> u32;
    fn increment_landed(&mut self, player: &Player) -> SpaceActionEnum;
}



struct BasicSpace {
    name : String,
    landed_count : u32
}
impl BasicSpace {
    fn new(name: String) -> Self {
        Self { name: name, landed_count: 0 }
    }
}
impl BoardSpace for BasicSpace {
    fn get_space_name(&self) -> &str {
        &self.name
    }

    fn get_landed_count(&self) -> u32 {
        self.landed_count
    }

    fn increment_landed(&mut self, player: &Player) -> SpaceActionEnum {
        self.landed_count += 1;
        println!("{} is on {}", player.get_player_name(), self.name);
        SpaceActionEnum::NoAction
    }
}



struct ChanceSpace {
    name : String,
    landed_count : u32
}
impl ChanceSpace {
    fn new() -> Self {
        Self { name: String::from("Chance"), landed_count: 0 }
    }
}
impl BoardSpace for ChanceSpace {
    fn get_space_name(&self) -> &str {
        &self.name
    }

    fn get_landed_count(&self) -> u32 {
        self.landed_count
    }

    fn increment_landed(&mut self, player: &Player) -> SpaceActionEnum {
        self.landed_count += 1;
        println!("{} is on {}", player.get_player_name(), self.name);
        SpaceActionEnum::DrawCard(CardDeckEnum::Chance)
    }
}



struct CommunityChestSpace {
    name : String,
    landed_count : u32
}
impl CommunityChestSpace {
    fn new() -> Self {
        Self { name: String::from("Community Chest"), landed_count: 0 }
    }
}
impl BoardSpace for CommunityChestSpace {
    fn get_space_name(&self) -> &str {
        &self.name
    }

    fn get_landed_count(&self) -> u32 {
        self.landed_count
    }

    fn increment_landed(&mut self, player: &Player) -> SpaceActionEnum {
        self.landed_count += 1;
        println!("{} is on {}", player.get_player_name(), self.name);
        SpaceActionEnum::DrawCard(CardDeckEnum::CommunityChest)
    }
}




struct GoToJailSpace {
    name : String,
    landed_count : u32
}
impl GoToJailSpace {
    fn new() -> Self {
        Self { name: String::from("Go To Jail"), landed_count: 0 }
    }
}
impl BoardSpace for GoToJailSpace {
    fn get_space_name(&self) -> &str {
        &self.name
    }

    fn get_landed_count(&self) -> u32 {
        self.landed_count
    }

    fn increment_landed(&mut self, player: &Player) -> SpaceActionEnum {
        self.landed_count += 1;
        println!("{} is on {}", player.get_player_name(), self.name);
        SpaceActionEnum::MovePlayer(10)
    }
}




fn initialize_game_board() -> HashMap::<u8, Box<dyn BoardSpace>> {
    let mut space_defs = HashMap::<u8, Box<dyn BoardSpace>>::with_capacity(40);


    space_defs.insert(0, Box::new(BasicSpace::new(String::from("Go"))));
    space_defs.insert(1, Box::new(BasicSpace::new(String::from("Mediterranean Avenue"))));
    space_defs.insert(2, Box::new(CommunityChestSpace::new()));
    space_defs.insert(3, Box::new(BasicSpace::new(String::from("Baltic Avenue"))));
    space_defs.insert(4, Box::new(BasicSpace::new(String::from("Income Tax"))));
    space_defs.insert(5, Box::new(BasicSpace::new(String::from("Reading Railroad"))));
    space_defs.insert(6, Box::new(BasicSpace::new(String::from("Oriental Avenue"))));
    space_defs.insert(7, Box::new(ChanceSpace::new()));
    space_defs.insert(8, Box::new(BasicSpace::new(String::from("Vermont Avenue"))));
    space_defs.insert(9, Box::new(BasicSpace::new(String::from("Connecticut Avenue"))));
    space_defs.insert(10, Box::new(BasicSpace::new(String::from("Jail"))));
    space_defs.insert(11, Box::new(BasicSpace::new(String::from("St. Charles Place"))));
    space_defs.insert(12, Box::new(BasicSpace::new(String::from("Electric Company"))));
    space_defs.insert(13, Box::new(BasicSpace::new(String::from("States Avenue"))));
    space_defs.insert(14, Box::new(BasicSpace::new(String::from("Virginia Avenue"))));
    space_defs.insert(15, Box::new(BasicSpace::new(String::from("Pennsylvania Railroad"))));
    space_defs.insert(16, Box::new(BasicSpace::new(String::from("St. James Place"))));
    space_defs.insert(17, Box::new(CommunityChestSpace::new()));
    space_defs.insert(18, Box::new(BasicSpace::new(String::from("Tennessee Avenue"))));
    space_defs.insert(19, Box::new(BasicSpace::new(String::from("New York Avenue"))));
    space_defs.insert(20, Box::new(BasicSpace::new(String::from("Free Parking"))));
    space_defs.insert(21, Box::new(BasicSpace::new(String::from("Kentucky Avenue"))));
    space_defs.insert(22, Box::new(ChanceSpace::new()));
    space_defs.insert(23, Box::new(BasicSpace::new(String::from("Indiana Avenue"))));
    space_defs.insert(24, Box::new(BasicSpace::new(String::from("Illinois Avenue"))));
    space_defs.insert(25, Box::new(BasicSpace::new(String::from("B & O Railroad"))));
    space_defs.insert(26, Box::new(BasicSpace::new(String::from("Atlantic Avenue"))));
    space_defs.insert(27, Box::new(BasicSpace::new(String::from("Ventnor Avenue"))));
    space_defs.insert(28, Box::new(BasicSpace::new(String::from("Waterworks"))));
    space_defs.insert(29, Box::new(BasicSpace::new(String::from("Marvin Gardens"))));
    space_defs.insert(30, Box::new(GoToJailSpace::new()));
    space_defs.insert(31, Box::new(BasicSpace::new(String::from("Pacific Avenue"))));
    space_defs.insert(32, Box::new(BasicSpace::new(String::from("North Carolina Avenue"))));
    space_defs.insert(33, Box::new(CommunityChestSpace::new()));
    space_defs.insert(34, Box::new(BasicSpace::new(String::from("Pennsylvania Avenue"))));
    space_defs.insert(35, Box::new(BasicSpace::new(String::from("Short Line Railroad"))));
    space_defs.insert(36, Box::new(ChanceSpace::new()));
    space_defs.insert(37, Box::new(BasicSpace::new(String::from("Park Place"))));
    space_defs.insert(38, Box::new(BasicSpace::new(String::from("Luxury Tax"))));
    space_defs.insert(39, Box::new(BasicSpace::new(String::from("Boardwalk"))));

    space_defs
}