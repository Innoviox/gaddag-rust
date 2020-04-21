import tkinter as tk

import subprocess
import base64
import string
import enum
import dataclasses

class Direction(enum.Enum):
    ACROSS = 1
    DOWN   = 2

    def flip(self):
        if self == Direction.ACROSS:
            return Direction.DOWN
        return Direction.ACROSS

    def __str__(self):
        return {Direction.ACROSS: '⇨', Direction.DOWN: '⇩'}[self]

    @property
    def x(self):
        return {Direction.ACROSS: 1, Direction.DOWN: 0}[self]

    @property
    def y(self):
        return {Direction.ACROSS: 0, Direction.DOWN: 1}[self]    

SPECIAL = {
    '#': 'red',
    '-': 'light blue',
    '+': 'dark blue',
    '^': 'pink',
    '*': 'pink'
}

@dataclasses.dataclass
class Puzzle:
    board: list
    rack: str
    moves: list
    
    @staticmethod
    def load_new(turns=10):
        puzzle = str(base64.b64decode(subprocess.check_output(["./target/release/gaddag-rust", "puzzle", str(turns)]).split()[-1]))[2:-1].replace(r"\n", "\n")
        board, rack, *moves = puzzle.split()
        board = [board[i:i+15].replace(".", " ") for i in range(0, len(board), 15)]

        return Puzzle(board, rack, moves)

class GUI:
    def __init__(self):
        self.puzzle = Puzzle.load_new()
        
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
                    t = self.puzzle.board[row - 1][col - 1]

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

        self.update_rack_frame()

        self.current_direction = Direction.ACROSS
        self.square_at = None
        self.squares_changed = []

        self.root.bind("<Key>", self.type_char)

    def update_rack_frame(self):
        self.rack_frame = tk.Frame(self.root, width=100, height=40, borderwidth=1, relief=tk.SUNKEN)

        for c, l in enumerate(self.puzzle.rack):
            frame = tk.Frame(self.rack_frame, width=20, height=20, borderwidth=1, relief=tk.GROOVE)
            tk.Label(frame, text=l).pack()
            frame.grid(row=0, column=c)
            
        self.rack_frame.grid(row=16, column=3, columnspan=10, rowspan=2)        

    def click(self, e):
        if e.widget['text'].strip().isalpha():
            return
        elif not self.square_at:
            self.square_at = e.widget
            self.squares_changed.append(e.widget)
        elif e.widget == self.square_at:
            self.current_direction = self.current_direction.flip()
        else:
            for sq in self.squares_changed:
                sq.config(text='', fg='black')
            self.square_at = e.widget
            self.squares_changed = [e.widget]
        e.widget['text'] = str(self.current_direction)

    def type_char(self, e):
        c = e.char.upper()
        if self.square_at and c in self.puzzle.rack:
            i = self.puzzle.rack.index(c)
            self.update_rack_frame()
            
            self.square_at.config(text=c, fg='brown')
            self.squares_changed.append(self.square_at)
            sq = None
            while self.square_at['text'].strip() and sq != self.square_at:
                sq, self.square_at = self.square_at, self.next_tile(self.square_at, self.current_direction)
            if sq != self.square_at:
                self.square_at.config(text=self.current_direction)
                self.squares_changed.append(self.square_at)

    def next_tile(self, tile, direction):
        a, b = [(row + direction.y, col + direction.x) for row, i in enumerate(self.labels) for col, j in enumerate(i) if j == tile][0]
        return self.labels[min(a, 15)][min(b, 15)]

GUI().root.mainloop()
