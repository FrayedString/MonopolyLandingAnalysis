use std::collections::HashMap;
use crate::game_simulation::player::Player;

pub enum CardDeckEnum {
    Chance,
    CommunityChest
}

pub enum SpaceActionEnum {
    NoAction,
    DrawCard(CardDeckEnum),
    MovePlayer(u8),
}




pub trait BoardSpace {
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




pub fn initialize_game_board() -> HashMap::<u8, Box<dyn BoardSpace>> {
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