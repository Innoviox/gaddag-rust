# gaddag-rust
aye matey, a rusty gaddag https://ericsink.com/downloads/faster-scrabble-gordon.pdf

### To get total time

```
sum(map(int, [i.split()[-1] for i in s.split("\n") if 'Time' in i]))
```

## Current UI Status

![example](/Users/chervjay/Documents/GitHub/gaddag-rust/example.png)

UI Roadmap

- clickable moves
- only highlight newly placed tiles
- customizable blanks, colors, everything
- shows considered moves
- placable moves
- more features