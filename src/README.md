TO-DO:

- Add sounds: welcome, move made, promotion, win/loss

- Re-do intro view graphic.

- Add difficulty/ai options to title view.

- Add forward/back action functionality.

- Add ability to restart game.

- Add rule about no checkmate by parachuting pawn?

- For reserve pieces, keep AI from evaluating identical reserve pieces.

- Reconsider MonteCarloTree implementation.

- Consider a Minimax / MonteCarlo hybrid where the evaluation function of Minimax
uses a random playout if depth is beyond a certain level and state is Ongoing. That would
avoid the need for a proper board evaluation function.
