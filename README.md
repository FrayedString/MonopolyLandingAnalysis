# MonopolyLandingAnalysis
Analyzing the odds of players landing on specific spaces in the classic Monopoly game and learning Rust all at the same time

Simulates dice rolls for a given number of players and # of turns each player will take and projects where those players would land on a classic Monopoly game board.  A 16 card-each deck system is implimented for Chance and Community Chest with the values taken from the US version here: https://www.monopolyland.com/list-monopoly-chance-community-chest-cards/  Chance and Community Chest cards that move a player to a new location are processed and their moves are included in the final space counts.


## Known Deficiencies
* While there are "get out of jail free" cards present in the Chance and Community Chest decks, and the fact of drawing them is output in the program execution.  There is no tracking as to whether or not the cards are currently in a players hands.  When/if the decks are re-shuffled, the cards are always re-included in the deck.
* Players immediately leave jail on their next roll after being sent to jail.


## Changelog
* 0.4.0 - Added pretty colors for property names.  Final results are now printed in order of most landed to least landed.  Number of players and turns to simulate are now prompted at runtime.  Updated crates.io dependencies to latest
* 0.3.0 - Refactored game logic from a single monolithic module (game_simulation) to 4 separate modules representing ownership of a distinct area of functionality
* 0.2.0 - Refactored app from a single main.rs file into a main.rs and game_simulation.rs module file that contains the simulation logic
* 0.1.0 - First ever release.  Entire codebase contained in main.rs, no use of traits or impls.  Basically it was terrible.


### Other Thoughts (legacy from v0.1.0 release)
This my my 3rd ever app in Rust.  The code is chaotic and, I have no doubt, quite deficient idiomatically speaking.  However it does seem to work, and I feel it was a great exercise for getting used to dealing with Rust error messages
