# Mini Shogi

Mini Shogi a simplified introduction to the Japanese game Shogi.

I created this app as a way to learn a little Shogi, but primarily Rust and Macroquad, the graphics framework.
<img width="684" alt="Screen Shot" src="https://user-images.githubusercontent.com/2192842/169587792-20082888-91da-4769-b5e3-61836ed225b0.png">

There are two AI opponents:

- (Enabled) Minimax. The classic "look x moves ahead" algorithm used in many perfect information situations. It has alpha-beta pruning, but it otherwise unsophisticated. On my computer, it can search about 1 million board positions per second, but it's still rather weak since the evaluation function is currently terrible. Be aware that looking ahead more than about 7 moves is rather slow.

- (Disabled) Monte Carlo. This algorithm makes each available move and then randomly plays out the game to completion a certain number of times (rounds). Due to its random nature, it plays with more variability than Minimax. This is "pure" Monte Carlo, a simple version that doesn't have the advantages of a full Monte Carlo Tree Search, which I hope to add later. It searches about 120,000 positions per second on my computer.

## To-Do:

- Improve minimax evaluation function.

- Add more sounds: welcome, win/loss.

- Add sound volume setting.

- Prevent AI from evaluating identical reserve pieces.

- Add rule about no checkmate by parachuting pawn? Doesn't seem particularly crucial though.

## Maybe

- Check out Macroquad's Texture2D.get_texture_data -> Image.get_pixel() for hit detection. Docs have warning: "This operation can be expensive."

- Check out Macroquad's 'set_pc_assets_folder' function.

- Add "radio" functionality to ButtonBar. Remove handling from ViewSettings.

- Add forward/back move functionality.

- Revisit MonteCarloTree implementation.

- Consider a Minimax / MonteCarlo hybrid where the evaluation function of Minimax uses a random playout if depth is beyond a certain level and state is Ongoing. That would avoid the need for a proper board evaluation function.
