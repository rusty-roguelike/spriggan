# spriggan
### An ASCII Roguelike
## Authors
Cody Potter, Alex Rose, Harsha Ramayanam
## Project Description
This project is an ascii roguelike made with the Rust programming language. 
The primary library we used was `rltk`, with the entity component system
provided by `specs`. 

Throughout the project we had numerous setbacks, which resulted in
starting from scratch on multiple occasions with different engines/frameworks/libraries.

We're happy with the direction this newest iteration is headed, even though
we didn't get to implement all the features we wanted.

### How to run the project
1. Install Rust
2. Clone this repo
3. Navigate to the directory
4. `cargo run`

#### How to play
Use the arrow keys to move around.
Press space to attack all the red monsters.
Clear the room.
Keep an eye on your HP in the console output.

#### Features not yet implemented
- Health potions to restore player health
- Player death
- Room advancement/Level up
- UI for current stats

### Testing
- We tested all features by hand before merging any features. 
- We wrote unit tests for functions that were outside the `rltk` and `specs` libraries.
- The ECS provided by specs makes testing library code difficult.
#### Known Issues
We ran out of time implementing the features we wanted.
As a result, there were some known bugs we didn't have time to fix.
- Monsters walk through walls

## Developer Journal (What Worked?)
The biggest takeaway I got from this project was to really research what libraries you use.
We ran into a number of issues with libraries that weren't really supported any longer.
In addition, some libraries have little-to-know helpful documentation or examples. This makes 
progress in a timed setting like this very difficult, especially if the entire team needs
to learn the library quickly. 

Just because a blog or reddit post hails a library as a great option, it doesn't mean
that library is currently supported, nor the links are still valid. It's very likely
to get pointed in the wrong direction. 

Big questions to ask:
- Are open issues getting resolved?
- Is it production ready?
- Are there good (up-to-date) examples/tutorials available?
- Are there a fair amount of developers using it?

Sometimes the borrow checker doesn't play nice with library code. There were a few times
when we had to go back to the tutorial/examples to see exactly how they were doing
certain things. Turns out, sometimes the tutorials had "hacks" to "make the borrow checker happy".

Going forward, I would continue to work with `rltk` and the `specs` entity component system.
The ECS model is a great way of decoupling logic in game development, and I will be using
the pattern going forward.

Note: we used the tutorial at https://bfnightly.bracketproductions.com/rustbook/ as a starting point.

## License
MIT
[Located here](LICENSE)
