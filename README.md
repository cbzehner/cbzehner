<div align="center">

  <h1><code>Chris Zehner's personal website</code></h1>

  <strong><a href="https://cbzehner.com">Try it out!</a></strong>
</div>

# Credits

Thanks to:
- TailwindCSS for the [color palette](https://tailwindcss.com/docs/customizing-colors)
- LogRocket for the [WASM & Rust starter kit](https://blog.logrocket.com/getting-started-with-webassembly-and-rust/)
- Rust Lang's WASM Working for an [amazing introductory book](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)
- Chris Coyier's article on [Old Timey Terminal Styling](https://css-tricks.com/old-timey-terminal-styling/)
- suggestions on styling the focus state https://moderncss.dev/custom-css-styles-for-form-inputs-and-textareas/#focus-state-styles
- https://terminalcss.xyz for the terminal prompt
- more vintage effects https://blog.carbonfive.com/vintage-terminal-effect-in-css3/

## Development Notes

### User Experience
- typing sounds
- black background 
- white text with blinking cursor 
- animation for input from the command prompt being transformed into output
- output text should have a "typing animation"

### Text Adventure Engine
#### Models 
- Game 
- Player 
- Input -> Action/Command
- Room 
- Object 
- Item (is object)
- Obstacle (is object)
- Event 

#### Nice-to-have features:
Logs of interactions 
Build time configuration 
Ability to send me an email
Help command 
Reset command and preserve game state in local storage 

#### Model Ideas
Rooms:
  - Office (Start) -> Garden | Tunnel Entrance | Secret Room
  - Garden (🎉) -> Office
  - Tunnel Entrance -> Office | Tunnel
  - Tunnel -> Tunnel Entrance | Tunnel Dead End
  - Tunnel Dead End -> Tunnel

Objects:
- Desk
  - business card (read/inspect)
  - Letter
  - Ancient Computer
    - Turn on/use (Note: _change font and color to green_
      - Would you like to play a game?
        - Conway’s Game of Life
        - Space Invaders
        - Snake
      - View Source code. Redirects to GitHub repo.
  - Drawers
    - Papers (read, take paperclip)
    - Torch
    - Pen (write)
    - Haig Club Whiskey & tumbler
    - USB drive
- Carpet -> Stairs -> Tunnel -> Grue OR sword & shield
- Bookcase -> Locked door -> Garden (tranquil music (listen), water feature (listen), hammock (take a nap)) OR if you have the sword ... !?!
- ???
