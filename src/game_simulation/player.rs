pub struct Player
{
    name: String,
    current_space: u8
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name, current_space: 0 }
    }

    pub fn set_current_space (&mut self, space: u8) {
        self.current_space = space;
    }

    pub fn get_current_space(&self) -> u8 {
        self.current_space
    }

    pub fn get_player_name(&self) -> &str {
        &self.name
    }
}