import tkinter as tk
import subprocess
import base64
import string

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



root = tk.Tk()

labels = []

for row in range(16):
    labels.append([])
    for col in range(16):
        if row == 0:
            t = (" " + string.ascii_uppercase)[col]
        elif col == 0:
            t = str(row).zfill(2)
        else:
            t = board[row - 1][col - 1]

        color = "white"
        if t in SPECIAL:
            color = SPECIAL[t]
            t = ''

        frame = tk.Frame(root, width=20, height=20, borderwidth=1, bg=color)
            
        if t: tk.Label(frame, text=t).pack()
        frame.grid(row=row, column=col)
        
        labels[-1].append(frame)

root.mainloop()
