use crate::game_simulation::player::Player;
use colored::{Colorize, ColoredString};

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
    fn get_space_name(&self) -> &ColoredString;
    fn get_landed_count(&self) -> u32;
    fn increment_landed(&mut self, player: &Player) -> SpaceActionEnum;
}



struct BasicSpace {
    name : ColoredString,
    landed_count : u32
}
impl BasicSpace {
    fn new(name: ColoredString) -> Self {
        Self { name: name, landed_count: 0 }
    }
}
impl BoardSpace for BasicSpace {
    fn get_space_name(&self) -> &ColoredString {
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
    name : ColoredString,
    landed_count : u32
}
impl ChanceSpace {
    fn new(idx: u8) -> Self {
        Self { name: format!("Chance ({})", idx).white().bold(), landed_count: 0 }
    }
}
impl BoardSpace for ChanceSpace {
    fn get_space_name(&self) -> &ColoredString {
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
    name : ColoredString,
    landed_count : u32
}
impl CommunityChestSpace {
    fn new(idx: u8) -> Self {
        Self { name: format!("Community Chest ({})", idx).white().bold(), landed_count: 0 }
    }
}
impl BoardSpace for CommunityChestSpace {
    fn get_space_name(&self) -> &ColoredString {
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
    name : ColoredString,
    landed_count : u32
}
impl GoToJailSpace {
    fn new() -> Self {
        Self { name: "Go To Jail".white().bold(), landed_count: 0 }
    }
}
impl BoardSpace for GoToJailSpace {
    fn get_space_name(&self) -> &ColoredString {
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




pub fn initialize_game_board() -> Vec::<Box<dyn BoardSpace>> {
    let mut space_defs = Vec::<Box<dyn BoardSpace>>::with_capacity(40);


    space_defs.insert(0, Box::new(BasicSpace::new("Go".white().bold())));
    space_defs.insert(1, Box::new(BasicSpace::new("Mediterranean Avenue".truecolor(138, 43, 226).bold())));
    space_defs.insert(2, Box::new(CommunityChestSpace::new(2)));
    space_defs.insert(3, Box::new(BasicSpace::new("Baltic Avenue".truecolor(138, 43, 226).bold())));
    space_defs.insert(4, Box::new(BasicSpace::new("Income Tax".white().bold())));
    space_defs.insert(5, Box::new(BasicSpace::new("Reading Railroad".white().bold())));
    space_defs.insert(6, Box::new(BasicSpace::new("Oriental Avenue".truecolor(135, 206, 250).bold())));
    space_defs.insert(7, Box::new(ChanceSpace::new(7)));
    space_defs.insert(8, Box::new(BasicSpace::new("Vermont Avenue".truecolor(135, 206, 250).bold())));
    space_defs.insert(9, Box::new(BasicSpace::new("Connecticut Avenue".truecolor(135, 206, 250).bold())));
    space_defs.insert(10, Box::new(BasicSpace::new("Jail".white().bold())));
    space_defs.insert(11, Box::new(BasicSpace::new("St. Charles Place".bright_magenta().bold())));
    space_defs.insert(12, Box::new(BasicSpace::new("Electric Company".truecolor(152, 251, 152).bold())));
    space_defs.insert(13, Box::new(BasicSpace::new("States Avenue".bright_magenta().bold())));
    space_defs.insert(14, Box::new(BasicSpace::new("Virginia Avenue".bright_magenta().bold())));
    space_defs.insert(15, Box::new(BasicSpace::new("Pennsylvania Railroad".white().bold())));
    space_defs.insert(16, Box::new(BasicSpace::new("St. James Place".truecolor(255, 140, 0).bold())));
    space_defs.insert(17, Box::new(CommunityChestSpace::new(17)));
    space_defs.insert(18, Box::new(BasicSpace::new("Tennessee Avenue".truecolor(255, 140, 0).bold())));
    space_defs.insert(19, Box::new(BasicSpace::new("New York Avenue".truecolor(255, 140, 0).bold())));
    space_defs.insert(20, Box::new(BasicSpace::new("Free Parking".white().bold())));
    space_defs.insert(21, Box::new(BasicSpace::new("Kentucky Avenue".red().bold())));
    space_defs.insert(22, Box::new(ChanceSpace::new(22)));
    space_defs.insert(23, Box::new(BasicSpace::new("Indiana Avenue".red().bold())));
    space_defs.insert(24, Box::new(BasicSpace::new("Illinois Avenue".red().bold())));
    space_defs.insert(25, Box::new(BasicSpace::new("B & O Railroad".white().bold())));
    space_defs.insert(26, Box::new(BasicSpace::new("Atlantic Avenue".yellow().bold())));
    space_defs.insert(27, Box::new(BasicSpace::new("Ventnor Avenue".yellow().bold())));
    space_defs.insert(28, Box::new(BasicSpace::new("Waterworks".truecolor(152, 251, 152).bold())));
    space_defs.insert(29, Box::new(BasicSpace::new("Marvin Gardens".yellow().bold())));
    space_defs.insert(30, Box::new(GoToJailSpace::new()));
    space_defs.insert(31, Box::new(BasicSpace::new("Pacific Avenue".truecolor(34, 139, 34).bold())));
    space_defs.insert(32, Box::new(BasicSpace::new("North Carolina Avenue".truecolor(34, 139, 34).bold())));
    space_defs.insert(33, Box::new(CommunityChestSpace::new(33)));
    space_defs.insert(34, Box::new(BasicSpace::new("Pennsylvania Avenue".truecolor(34, 139, 34).bold())));
    space_defs.insert(35, Box::new(BasicSpace::new("Short Line Railroad".white().bold())));
    space_defs.insert(36, Box::new(ChanceSpace::new(36)));
    space_defs.insert(37, Box::new(BasicSpace::new("Park Place".blue().bold())));
    space_defs.insert(38, Box::new(BasicSpace::new("Luxury Tax".white().bold())));
    space_defs.insert(39, Box::new(BasicSpace::new("Boardwalk".blue().bold())));

    space_defs
}