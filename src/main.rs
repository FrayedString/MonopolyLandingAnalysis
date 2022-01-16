use futures::executor::block_on;

mod game_objects;
use game_objects::{GameSimulation};


fn main() {
    
    let mut g = GameSimulation::new(4);
    let work = g.run_simulation(30);

    block_on(work);
}
