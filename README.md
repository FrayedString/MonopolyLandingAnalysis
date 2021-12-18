# MonopolyLandingAnalysis
Analyzing the odds of players landing on specific spaces in the classic Monopoly game and learning Rust all at the same time

Simulates dice rolls for a given number of players and # of turns each player will take and projects where those players would land on a classic Monopoly game board.  A 16 card-each deck system is implimented for Chance and Community Chest with the values taken from the US version here: https://www.monopolyland.com/list-monopoly-chance-community-chest-cards/  Chance and Community Chest cards that move a player to a new location are processed and their moves are included in the final space counts.

Some sample results can be viewed here: https://docs.google.com/spreadsheets/d/1QLPWaPdggRG7qy4VrLI8ymML9f6uvS5FN2b1l3VdaeE/edit?usp=sharing


## Known Deficiencies
* While there are "get out of jail free" cards present in the Chance and Community Chest decks, and the fact of drawing them is output in the program execution.  There is no tracking as to whether or not the cards are currently in a players hands.  When/if the decks are re-shuffled, the cards are always re-included in the deck.
* Simulated players are not sent to jail upon rolling doubles 3x in a row


### Other Thoughts
This my my 3rd ever app in Rust.  The code is chaotic and, I have no doubt, quite deficient idiomatically speaking.  However it does seem to work, and I feel it was a great exercise for getting used to dealing with Rust error messages
