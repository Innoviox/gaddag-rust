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
        board, rack, *moves = puzzle.split("\n")
        rack = list(rack)
        
        board = [board[i:i+15].replace(".", " ") for i in range(0, len(board), 15)]

        return Puzzle(board, rack, moves)

    def rank_of_move(self, position, word, direction):
        y, x = position
        c, r = str(y), string.ascii_uppercase[x - 1]
        s = [r + c, c + r][direction==Direction.ACROSS] + ' ' + word + ' '
        try:
            move = [i for i in self.moves if i.startswith(s)][0]
            r = self.moves.index(move) + 1
            return r
        except IndexError:
            # print("Invalid move", s, "(or not in top 500)")
            return None

# thanks https://gist.github.com/mp035/9f2027c3ef9172264532fcd6262f3b01 
class ScrollFrame(tk.Frame):
    def __init__(self, parent):
        super().__init__(parent) # create a frame (self)

        self.canvas = tk.Canvas(self, borderwidth=0, background="#ffffff")          #place canvas on self
        self.viewPort = tk.Frame(self.canvas, background="#ffffff")                    #place a frame on the canvas, this frame will hold the child widgets 
        self.vsb = tk.Scrollbar(self, orient="vertical", command=self.canvas.yview) #place a scrollbar on self 
        self.canvas.configure(yscrollcommand=self.vsb.set)                          #attach scrollbar action to scroll of canvas

        self.vsb.pack(side="right", fill="y")                                       #pack scrollbar to right of self
        self.canvas.pack(side="left", fill="both", expand=True)                     #pack canvas to left of self and expand to fil
        self.canvas_window = self.canvas.create_window((4,4), window=self.viewPort, anchor="nw",            #add view port frame to canvas
                                  tags="self.viewPort")

        self.viewPort.bind("<Configure>", self.onFrameConfigure)                       #bind an event whenever the size of the viewPort frame changes.
        self.canvas.bind("<Configure>", self.onCanvasConfigure)                       #bind an event whenever the size of the viewPort frame changes.

        self.onFrameConfigure(None)                                                 #perform an initial stretch on render, otherwise the scroll region has a tiny border until the first resize

    def onFrameConfigure(self, event):                                              
        '''Reset the scroll region to encompass the inner frame'''
        self.canvas.configure(scrollregion=self.canvas.bbox("all"))                 #whenever the size of the frame changes, alter the scroll region respectively.

    def onCanvasConfigure(self, event):
        '''Reset the canvas window to encompass inner frame when required'''
        canvas_width = event.width
        self.canvas.itemconfig(self.canvas_window, width = canvas_width)            #whenever the size of the canvas changes alter the window region respectively.



class GUI:
    def __init__(self):
        self.puzzle = Puzzle.load_new()
        self.original_rack = self.puzzle.rack[:]
        
        self.root = tk.Tk()

        self.board_frame = tk.Frame(self.root, borderwidth=1, relief=tk.RIDGE)

        self.labels = []

        for row in range(16):
            self.labels.append([])
            for col in range(16):
                color, fg = "white", "black"

                bw = 1
                relief=tk.FLAT
                
                if row == 0:
                    t = (" " + string.ascii_uppercase)[col]
                elif col == 0:
                    t = str(row).zfill(2)
                else:
                    t = self.puzzle.board[row - 1][col - 1]
                    if t.isalpha() and t == t.lower():
                        relief=tk.RIDGE
                        t = t.upper()
                        
                if t in SPECIAL:
                    color = SPECIAL[t]
                    t = ''

                frame = tk.Frame(self.board_frame, width=20, height=20, borderwidth=bw, relief=relief, bg=color)

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
        self.root.bind("<Return>", self.enter_move)

        self.move_box = ScrollFrame(self.root)
        self.move_box.grid(row=0, column=16, rowspan=8, columnspan=2)

        self.move_btns = []
        self.ml = max(map(len, self.puzzle.moves))
        for i in range(1, len(self.puzzle.moves)):
            btn = tk.Button(self.move_box.viewPort, text=str(i), command=self.show(i - 1), font='TkFixedFont')
            btn.pack(fill=tk.X)

            self.move_btns.append(btn)

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
            self.puzzle.rack = self.original_rack[:]
            self.square_at = e.widget
            self.squares_changed = [e.widget]
        e.widget['text'] = str(self.current_direction)

    def type_char(self, e):
        c = e.char.upper()
        if self.square_at and c in self.puzzle.rack:
            self.puzzle.rack.remove(c)
            
            self.square_at.config(text=c, fg='brown')
            self.squares_changed.append(self.square_at)
            sq = None
            while self.square_at['text'].strip() and sq != self.square_at:
                sq, self.square_at = self.square_at, self.next_tile(self.square_at, self.current_direction)
            if sq != self.square_at:
                self.square_at.config(text=self.current_direction)
                self.squares_changed.append(self.square_at)

    def location_of(self, tile):
        return [(row, col) for row, i in enumerate(self.labels) for col, j in enumerate(i) if j == tile][0]

    def enter_move(self, e):
        first = self.squares_changed[0]
        loc = self.location_of(first)
        word = ''
        sq = None
        while first['text'].isalpha() and sq != first:
            if first in self.squares_changed:
                word += first['text']
            else:
                word += '(' + first['text'] + ')'
            sq, first = first, self.next_tile(first, self.current_direction)
        word = word.replace(')(', '')
        
        if r := self.puzzle.rank_of_move(loc, word, self.current_direction):
            self.show(r - 1)()

    def next_tile(self, tile, direction):
        a, b = self.location_of(tile)
        return self.labels[min(a + direction.y, 15)][min(b + direction.x, 15)]

    def show(self, i):
        def clicked():
            self.move_btns[i]['text'] = (str(i + 1).zfill(3) + '. ' + self.puzzle.moves[i]).ljust(self.ml + 5)
        return clicked

g = GUI()
g.root.mainloop()


'''
check! todo: blanks on board
todo: blanks on rack
todo: reveal top move
todo: backspace
'''
