import tkinter as tk

import subprocess
import base64
import string
import enum

class Direction(enum.Enum):
    ACROSS = 1
    DOWN   = 2

    def flip(self):
        if self == Direction.ACROSS:
            return Direction.DOWN
        return Direction.ACROSS

    def __str__(self):
        return {Direction.ACROSS: '⇨', Direction.DOWN: '⇩'}[self]

SPECIAL = {
    '#': 'red',
    '-': 'light blue',
    '+': 'dark blue',
    '^': 'pink',
    '*': 'pink'
}

puzzle = str(base64.b64decode(subprocess.check_output(["./target/release/gaddag-rust", "puzzle", "10"]).split()[-1]))[2:-1].replace(r"\n", "\n")
board, rack, *moves = puzzle.split()
board = [board[i:i+15].replace(".", " ") for i in range(0, len(board), 15)]

class Puzzle:
    def __init__(self):
        self.root = tk.Tk()

        self.board_frame = tk.Frame(self.root, borderwidth=1, relief=tk.RIDGE)

        self.labels = []

        for row in range(16):
            self.labels.append([])
            for col in range(16):
                color, fg = "white", "black"
                
                if row == 0:
                    t = (" " + string.ascii_uppercase)[col]
                elif col == 0:
                    t = str(row).zfill(2)
                else:
                    t = board[row - 1][col - 1]

                if t in SPECIAL:
                    color = SPECIAL[t]
                    t = ''

                frame = tk.Frame(self.board_frame, width=20, height=20, borderwidth=1, bg=color)

                label = tk.Label(frame, text=t, bg=color)
                label.pack()

                frame.pack_propagate(0)
                frame.grid(row=row, column=col)

                label.bind("<Button-1>", self.click)
                
                self.labels[-1].append(label)
        self.board_frame.grid(row=0, column=0, rowspan=16, columnspan=16)

        self.rack_frame = tk.Frame(self.root, width=100, height=40, borderwidth=1, relief=tk.SUNKEN)

        for c, l in enumerate(rack):
            frame = tk.Frame(self.rack_frame, width=20, height=20, borderwidth=1, relief=tk.GROOVE)
            tk.Label(frame, text=l).pack()
            frame.grid(row=0, column=c)
            
        self.rack_frame.grid(row=16, column=3, columnspan=10, rowspan=2)

        self.current_direction = Direction.ACROSS
        self.square_at = None

        self.root.bind("<Key>", self.type_char)

    def click(self, e):
        set_text = False
        if not self.square_at and not e.widget['text'].strip():
            self.square_at = e.widget
            set_text = True
        elif e.widget == self.square_at:
            self.current_direction = self.current_direction.flip()
            set_text = True
        else:
            for sq in self.squares_changed:
                sq.config(text='', fg='black')
            self.square_at = e.widget
            self.squares_changed = []
        if set_text:
            e.widget['text'] = str(self.current_direction)

    def type_char(self, e):
        if self.square_at:
            self.square_at.config(text=e.char.upper(), fg='yellow')
            self.squares_changed.append(self.square_at)

Puzzle().root.mainloop()
