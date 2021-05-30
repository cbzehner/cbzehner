<div align="center">

  <h1><code>Chris Zehner's personal website</code></h1>

  <strong><a href="https://cbzehner.com">Try it out!</a></strong>
</div>

## Development Notes

### User Experience
- typing sounds
- black background 
- white text with blinking cursor 
- transformer input animation 

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
