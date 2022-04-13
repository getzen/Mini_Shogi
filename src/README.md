# Yōkaï

This game is based on the board game Yōkaï No Mori, a boardgame published by Ferti:
https://boardgamegeek.com/boardgame/148641/ykai-no-mori

Yōkaï No Mori is, in turn, a simplified introduction to the Japanese game Shogi. The board game has "classic" rules (beginniner) and expert rules. This app uses the expert rules.

I created this app as a way to learn both Rust and Macroquad, the graphics framework. Rather than use the original artwork from the game, I created my own westernized Shogi pieces and rather plain board.

There are two AI opponents:

- Minimax. The classic "look x moves ahead" algorithm used in many perfect information situations. It has alpha-beta pruning, but it otherwise unsophisticated. On my computer, it can search about 900,000 board positions per second, but it's still rather weak since the evaluation function is currently terrible. Be aware that looking ahead more than about 7 moves is rather slow. In this app, Minimax is generally stronger than Monte Carlo.

- Monte Carlo. This algorithm makes each available move and then randomly plays out the game to completion a certain number of times (rounds). Due to its random nature, it plays with more variability than Minimax. This is "pure" Monte Carlo, a simple version that doesn't have the advantages of a full Monte Carlo Tree Search, which I hope to add later. It searches about 120,000 positions per second on my computer.

## To-Do:

- Add rules view.

- Add sounds: welcome, promotion, win/loss

- Add ability to restart game.
 
- Add forward/back move functionality.

- Add rule about no checkmate by parachuting pawn? Doesn't seem particularly crucial.

- For reserve pieces, keep AI from evaluating identical reserve pieces.

- Revisit MonteCarloTree implementation.

- Consider a Minimax / MonteCarlo hybrid where the evaluation function of Minimax
uses a random playout if depth is beyond a certain level and state is Ongoing. That would
avoid the need for a proper board evaluation function.
