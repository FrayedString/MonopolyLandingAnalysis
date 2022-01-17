use std::collections::VecDeque;

use rand::{prelude::ThreadRng, Rng};

//Represents the move action to be taken when a Chance or community chest card is drawn
type DrawnCardAction = fn(u8) -> Option<u8>;

#[derive(PartialEq)]
pub struct GameActionCard
{
    text: String,
    move_action: DrawnCardAction
}
impl GameActionCard {
    pub fn get_card_text(&self) -> &str {
        &self.text
    }

    pub fn get_move_action(&self) -> &DrawnCardAction {
        &self.move_action
    }
}


pub struct CardDecks 
{
    chance_deck: GameActionCardDeck,
    community_chest_deck: GameActionCardDeck,
}
impl CardDecks {
    pub fn new() -> Self {
        CardDecks { 
            chance_deck: init_chance(),
            community_chest_deck: init_community_chest()
        }
    }

    pub fn get_chance_deck(&mut self) -> &mut GameActionCardDeck {
        &mut self.chance_deck
    }

    pub fn get_community_chest_deck(&mut self) -> &mut GameActionCardDeck {
        &mut self.community_chest_deck
    }
}




pub struct GameActionCardDeck
{
    deck: VecDeque<GameActionCard>,
    dealt: Vec<GameActionCard>
}

impl GameActionCardDeck {
    fn new(cards: Vec::<GameActionCard>) -> Self {
        //Initialize cards into the Dealt vec, deck will shuffle on first use
        GameActionCardDeck { deck: VecDeque::<GameActionCard>::with_capacity(16), dealt: cards }
    }


    pub fn draw_card(&mut self, rng: &mut ThreadRng) -> Option<&GameActionCard> {
        //Shuffle if needed
        if self.deck.len() == 0 {
            //let mut rng = thread_rng();

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