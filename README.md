# gaddag-rust
aye matey, a rusty gaddag https://ericsink.com/downloads/faster-scrabble-gordon.pdf

## Building and running

``` bash
$ cargo run --release (text|viz)
```

Default argument is viz. Both will run a single AI game.



### To get total time

```
sum(map(int, [i.split()[-1] for i in s.split("\n") if 'Time' in i]))
```

## Current UI Status

Initial UI

![initial ui december 1st](ui/12_1_19_20_28.png)

Scores, Clickable moves

![december 3rd with scores and move buttons](ui/12_3_19_17_39.png)

Boxed Blanks, shrunk moves

![december 3rd with boxed blanks](ui/12_3_19_22_16.png)

Rack, graph

![afopr](ui/12_5_19_20_41.png)

UI Roadmap

- clickable moves :white_check_mark:
  - make look like labels/less big
- only highlight newly placed tiles
- customizable blanks, colors, everything
- shows considered moves
- placable moves
- more features