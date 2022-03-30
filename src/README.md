TO-DO:

- Add sounds: welcome, move made, promotion, win/loss

- Add difficulty/ai options to title view.

- Add forward/back action functionality.

- Add ability to restart game.

- Add rule about parachuting pawn onto same column as another of player's pawns.
- Add rule about no checkmate by parachuting pawn?

- For reserve pieces, have one spot per piece kind. Keep AI from evaluating identical
reserve pieces.

- Consider a Minimax / MonteCarlo hybrid where the evaluation function of Minimax
uses a random playout if depth is beyond a certain level and state is Ongoing. That would
avoid the need for a proper board evaluation function.
