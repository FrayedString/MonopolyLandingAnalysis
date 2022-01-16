use futures::executor::block_on;

mod game_simulation;


fn main() {
    
    //let mut g = GameSimulation::new(4);
    let work = game_simulation::run_simulation(4, 30);

    block_on(work);
}
