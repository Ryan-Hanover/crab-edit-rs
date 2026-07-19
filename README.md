# crab-edit-rs
TUI Rust Text-Editor, No AI Code

- [ ] - Read input from a file
- [ ] - Implement cursor control with arrow keys
- [ ] - Implement cursor control with mouse

Actually Fuck the current implementation, we must adjsut

### We need
- A buffer struct, this contains all the data/lines and defines all operations on it, totally sepreate from all IO
- Positon struct, probably just a usize for both row and col, but it be nice to define stuff on it
- Cursor struct, it should just contain a position and "desiried col", desiried col being the place the we should jump to when a line is long enough
- A terminal struct, basically what my RAII guard does now combined with holding stdout
- Editor: This is what relates all these things
- Main of-coures

### Plan of Action
- Port this buggy implementation with the Vec<string> version, once done I will port to a real text editor data structure, gotta do some house keeping
